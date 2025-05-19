mod audio_files;
use audio_files::*;
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::sync::{Arc, Mutex, atomic::{AtomicUsize, Ordering}};
use std::env;
use std::path::{Path, PathBuf};
use slint::{Weak, ComponentHandle}; // Add this for Slint handles

slint::include_modules!();

pub struct AudioToggle {
    sink: Option<Arc<Sink>>,
    file: PathBuf,
    stream_handle: OutputStreamHandle,
    playback_id: Arc<AtomicUsize>,
}

impl AudioToggle {
    pub fn new(file: PathBuf, stream_handle: OutputStreamHandle) -> Self {
        Self {
            sink: None,
            file,
            stream_handle,
            playback_id: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn is_playing(&self) -> bool {
        self.sink.as_ref().map_or(false, |sink| !sink.empty())
    }

    /// `ui_weak` is a Weak<MainWindow>
    /// `set_indicator` is a property setter, e.g., MainWindow::set_track1_playing
    pub fn toggle(
        &mut self,
        ui_weak: Weak<MainWindow>,
        set_indicator: fn(&MainWindow, bool),
    ) {
        // Stop current playback if playing
        if let Some(sink) = &self.sink {
            if !sink.empty() {
                sink.stop();
                self.sink = None;
                // Invalidate any running threads
                self.playback_id.fetch_add(1, Ordering::SeqCst);
                let ui_weak_copy = ui_weak.clone();
                slint::invoke_from_event_loop(move || {
                    if let Some(ui) = ui_weak_copy.upgrade() {
                        set_indicator(&ui, false);
                    }
                }).unwrap();
                return;
            }
        }

        // Start new playback
        println!("Trying to open file: {}", self.file.display());
        let file = match File::open(&self.file) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Failed to open audio file: {}", e);
                let ui_weak_copy = ui_weak.clone();
                slint::invoke_from_event_loop(move || {
                    if let Some(ui) = ui_weak_copy.upgrade() {
                        set_indicator(&ui, false);
                    }
                }).unwrap();
                return;
            }
        };
        let source = match Decoder::new(BufReader::new(file)) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to decode audio: {}", e);
                let ui_weak_copy = ui_weak.clone();
                slint::invoke_from_event_loop(move || {
                    if let Some(ui) = ui_weak_copy.upgrade() {
                        set_indicator(&ui, false);
                    }
                }).unwrap();
                return;
            }
        };
        let sink = match Sink::try_new(&self.stream_handle) {
            Ok(sink) => Arc::new(sink),
            Err(e) => {
                eprintln!("Failed to create sink: {}", e);
                let ui_weak_copy = ui_weak.clone();
                slint::invoke_from_event_loop(move || {
                    if let Some(ui) = ui_weak_copy.upgrade() {
                        set_indicator(&ui, false);
                    }
                }).unwrap();
                return;
            }
        };
        sink.append(source);

        // Set indicator ON on main thread
        let ui_weak_copy = ui_weak.clone();
        slint::invoke_from_event_loop(move || {
            if let Some(ui) = ui_weak_copy.upgrade() {
                set_indicator(&ui, true);
            }
        }).unwrap();

        // Handle indicator OFF when playback ends (only if still current)
        let playback_id = self.playback_id.fetch_add(1, Ordering::SeqCst) + 1;
        let playback_id_arc = self.playback_id.clone();
        let sink_thread = Arc::clone(&sink);
        let ui_weak_thread = ui_weak.clone();

        std::thread::spawn(move || {
            sink_thread.sleep_until_end();
            if playback_id == playback_id_arc.load(Ordering::SeqCst) {
                let ui_weak_copy = ui_weak.clone();
                slint::invoke_from_event_loop(move || {
                    if let Some(ui) = ui_weak_copy.upgrade() {
                        set_indicator(&ui, false);
                    }
                }).unwrap();
            }
        });

        self.sink = Some(sink);
    }
}

fn build_path(dir: &Path, file: &str) -> PathBuf {
    dir.join(file)
}

fn main() -> Result<(), slint::PlatformError> {
    let dir = env::args().nth(1).expect("Usage: program <sound_directory>");
    let dir_path = Path::new(&dir);

    let (_stream, stream_handle) = OutputStream::try_default().expect("No audio output device");

    let main_window = MainWindow::new()?;

    let opening_procession = Arc::new(Mutex::new(AudioToggle::new(build_path(dir_path, OPENING_PROCESSION), stream_handle.clone())));
    let open_great_lights = Arc::new(Mutex::new(AudioToggle::new(build_path(dir_path, OPEN_GREAT_LIGHTS), stream_handle.clone())));
    let rimshot1 = Arc::new(Mutex::new(AudioToggle::new(build_path(dir_path, RIMSHOT1), stream_handle.clone())));
    let rimshot2 = Arc::new(Mutex::new(AudioToggle::new(build_path(dir_path, RIMSHOT2), stream_handle.clone())));
    let rimshot3 = Arc::new(Mutex::new(AudioToggle::new(build_path(dir_path, RIMSHOT3), stream_handle.clone())));
    let rimshot4 = Arc::new(Mutex::new(AudioToggle::new(build_path(dir_path, RIMSHOT4), stream_handle.clone())));


    // Helper to get weak handle for indicator updates
    let ui_weak = main_window.as_weak();

    // Macro to reduce repetition
    macro_rules! connect_button {
        ($toggle:ident, $set_prop:expr, $cb:ident) => {
            {
                let toggle = $toggle.clone();
                let ui_weak = ui_weak.clone();
                main_window.$cb(move || {
                    toggle.lock().unwrap().toggle(ui_weak.clone(), $set_prop);
                });
            }
        };
    }

    connect_button!(opening_procession, MainWindow::set_opening_procession_playing, on_toggle_opening_procession);
    connect_button!(open_great_lights, MainWindow::set_open_great_lights_playing, on_toggle_open_great_lights);
    connect_button!(rimshot1, MainWindow::set_rimshot1_playing, on_toggle_rimshot1);
    connect_button!(rimshot2, MainWindow::set_rimshot2_playing, on_toggle_rimshot2);
    connect_button!(rimshot3, MainWindow::set_rimshot3_playing, on_toggle_rimshot3);
    connect_button!(rimshot4, MainWindow::set_rimshot4_playing, on_toggle_rimshot4);


    // {
    //     let opening_procession = opening_procession.clone();
    //     let main_window_weak = main_window.as_weak();
    //     main_window.on_toggle_opening_procession(move || {
    //         let mut adio = opening_procession.lock().unwrap();
    //         audio.toggle();
    //         if let Some(main_window) = main_window_weak.upgrade() {
    //             main_window.set_opening_procession_playing(audio.is_playing());
    //         }
    //     });
    // }
    // {
    //     let open_great_lights = open_great_lights.clone();
    //     main_window.on_toggle_open_great_lights(move || {
    //         open_great_lights.lock().unwrap().toggle();
    //     });
    // }
    // {
    //     let rimshot1 = rimshot1.clone();
    //     main_window.on_toggle_rimshot1(move || {
    //         rimshot1.lock().unwrap().toggle();
    //     });
    // }
    // {
    //     let rimshot2 = rimshot2.clone();
    //     main_window.on_toggle_rimshot2(move || {
    //         rimshot2.lock().unwrap().toggle();
    //     });
    // }
    // {
    //     let rimshot3 = rimshot3.clone();
    //     main_window.on_toggle_rimshot3(move || {
    //         rimshot3.lock().unwrap().toggle();
    //     });
    // }
    // {
    //     let rimshot4 = rimshot4.clone();
    //     main_window.on_toggle_rimshot4(move || {
    //         rimshot4.lock().unwrap().toggle();
    //     });
    // }

    main_window.run()
}
