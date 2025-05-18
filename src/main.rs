mod audio_files;
use audio_files::*;
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};

use std::env;
use std::path::{Path, PathBuf};

slint::include_modules!();

struct AudioToggle {
    sink: Option<Sink>,
    file: PathBuf,
    stream_handle: OutputStreamHandle,
}

impl AudioToggle {
    fn new(file: PathBuf, stream_handle: OutputStreamHandle) -> Self {
        Self {
            sink: None,
            file,
            stream_handle,
        }
    }

    fn toggle(&mut self) {
        if let Some(sink) = &self.sink {
            if !sink.empty() {
                sink.stop();
                self.sink = None;
                return;
            }
        }
        println!("Trying to open file: {}", self.file.display());
        let file = File::open(&self.file).expect("Failed to open audio file");
        let source = Decoder::new(BufReader::new(file)).expect("Failed to decode audio");
        let sink = Sink::try_new(&self.stream_handle).expect("Failed to create sink");
        sink.append(source);
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

    let opening_procession = Arc::new(Mutex::new(AudioToggle::new(build_path(dir_path, OPENING_PROCESSION), stream_handle.clone())));
    let open_great_lights = Arc::new(Mutex::new(AudioToggle::new(build_path(dir_path, OPEN_GREAT_LIGHTS), stream_handle.clone())));
    let rimshot1 = Arc::new(Mutex::new(AudioToggle::new(build_path(dir_path, RIMSHOT1), stream_handle.clone())));
    let rimshot2 = Arc::new(Mutex::new(AudioToggle::new(build_path(dir_path, RIMSHOT2), stream_handle.clone())));
    let rimshot3 = Arc::new(Mutex::new(AudioToggle::new(build_path(dir_path, RIMSHOT3), stream_handle.clone())));
    let rimshot4 = Arc::new(Mutex::new(AudioToggle::new(build_path(dir_path, RIMSHOT4), stream_handle.clone())));

    let main_window = MainWindow::new()?;

    {
        let opening_procession = opening_procession.clone();
        main_window.on_toggle_opening_procession(move || {
            opening_procession.lock().unwrap().toggle();
        });
    }
    {
        let open_great_lights = open_great_lights.clone();
        main_window.on_toggle_open_great_lights(move || {
            open_great_lights.lock().unwrap().toggle();
        });
    }
    {
        let rimshot1 = rimshot1.clone();
        main_window.on_toggle_rimshot1(move || {
            rimshot1.lock().unwrap().toggle();
        });
    }
    {
        let rimshot2 = rimshot2.clone();
        main_window.on_toggle_rimshot2(move || {
            rimshot2.lock().unwrap().toggle();
        });
    }
    {
        let rimshot3 = rimshot3.clone();
        main_window.on_toggle_rimshot3(move || {
            rimshot3.lock().unwrap().toggle();
        });
    }
    {
        let rimshot4 = rimshot4.clone();
        main_window.on_toggle_rimshot4(move || {
            rimshot4.lock().unwrap().toggle();
        });
    }

    main_window.run()
}
