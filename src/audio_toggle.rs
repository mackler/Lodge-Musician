use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStreamHandle, Sink};
use std::sync::atomic::{AtomicUsize, Ordering};
use slint::Weak;
use std::sync::Arc;
use std::path::PathBuf;
use crate::MainWindow;

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
