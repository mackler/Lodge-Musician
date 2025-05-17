use eframe::egui;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::fs::File;
use std::io::BufReader;

struct AudioToggle {
    sink: Option<Sink>,
    file: &'static str,
    stream_handle: OutputStreamHandle,
}

impl AudioToggle {
    fn new(file: &'static str, stream_handle: OutputStreamHandle) -> Self {
        AudioToggle {
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
        let file = File::open(self.file).expect("Failed to open file");
        let source = Decoder::new(BufReader::new(file)).expect("Failed to decode audio");
        let sink = Sink::try_new(&self.stream_handle).expect("Failed to create sink");
        sink.append(source);
        self.sink = Some(sink);
    }
}

struct MyApp {
    audio1: AudioToggle,
    audio2: AudioToggle,
}

impl MyApp {
    fn new(stream_handle: OutputStreamHandle) -> Self {
        MyApp {
            audio1: AudioToggle::new("/home/mackler/Music/artists/Aerosmith - Complete Discography/1973 - Aerosmith/01. Make It.mp3", stream_handle.clone()),
            audio2: AudioToggle::new("/home/mackler/Music/masonic-music/files-for-work/more-outro.wav", stream_handle),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Toggle Audio 1").clicked() {
                self.audio1.toggle();
            }
            if ui.button("Toggle Audio 2").clicked() {
                self.audio2.toggle();
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    let (_stream, stream_handle) = OutputStream::try_default().expect("No audio output device");
    let app = MyApp::new(stream_handle);
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Audio Toggle Example",
        native_options,
        Box::new(|_cc| Box::new(app)),
    )
}
