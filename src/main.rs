use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};

use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};

slint::include_modules!();

struct AudioToggle {
    sink: Option<Sink>,
    file: &'static str,
    stream_handle: OutputStreamHandle,
}

impl AudioToggle {
    fn new(file: &'static str, stream_handle: OutputStreamHandle) -> Self {
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
        let file = File::open(self.file).expect("Failed to open audio file");
        let source = Decoder::new(BufReader::new(file)).expect("Failed to decode audio");
        let sink = Sink::try_new(&self.stream_handle).expect("Failed to create sink");
        sink.append(source);
        self.sink = Some(sink);
    }
}

fn main() -> Result<(), slint::PlatformError> {
    let (_stream, stream_handle) = OutputStream::try_default().expect("No audio output device");

    let audio1 = Arc::new(Mutex::new(AudioToggle::new("/home/mackler/Music/masonic-music/files-for-work/more-outro.wav", stream_handle.clone())));
    let audio2 = Arc::new(Mutex::new(AudioToggle::new("/home/mackler/Music/artists/Aerosmith - Complete Discography/1973 - Aerosmith/01. Make It.mp3", stream_handle)));

    let main_window = MainWindow::new()?;

    {
        let audio1 = audio1.clone();
        main_window.on_toggle_audio1(move || {
            audio1.lock().unwrap().toggle();
        });
    }
    {
        let audio2 = audio2.clone();
        main_window.on_toggle_audio2(move || {
            audio2.lock().unwrap().toggle();
        });
    }

    main_window.run()
}
