mod audio_files;
use audio_files::*;
use rodio::OutputStream;
use std::sync::{Arc, Mutex};
use std::env;
use std::path::{Path, PathBuf};
// use slint::ComponentHandle;

mod audio_toggle;
use audio_toggle::AudioToggle;

slint::include_modules!();


fn build_path(dir: &Path, file: &str) -> PathBuf {
    dir.join(file)
}

fn main() -> Result<(), slint::PlatformError> {
    let dir = env::args().nth(1).expect("Usage: program <sound_directory>");
    let dir_path = Path::new(&dir);

    let (_stream, stream_handle) = OutputStream::try_default().expect("No audio output device");

    let main_window = MainWindow::new()?;

    let opening_procession = Arc::new(Mutex::new(AudioToggle::new(build_path(dir_path, OPENING_PROCESSION), stream_handle.clone())));
    let national_anthem = Arc::new(Mutex::new(AudioToggle::new(build_path(dir_path, NATIONAL_ANTHEM), stream_handle.clone())));
    let open_tapis = Arc::new(Mutex::new(AudioToggle::new(build_path(dir_path, OPEN_TAPIS), stream_handle.clone())));
    let open_great_lights = Arc::new(Mutex::new(AudioToggle::new(build_path(dir_path, OPEN_GREAT_LIGHTS), stream_handle.clone())));
    let mystic_chain = Arc::new(Mutex::new(AudioToggle::new(build_path(dir_path, MYSTIC_CHAIN), stream_handle.clone())));
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
    connect_button!(national_anthem, MainWindow::set_national_anthem_playing, on_toggle_national_anthem);
    connect_button!(open_tapis, MainWindow::set_open_tapis_playing, on_toggle_open_tapis);
    connect_button!(open_great_lights, MainWindow::set_open_great_lights_playing, on_toggle_open_great_lights);
    connect_button!(mystic_chain, MainWindow::set_mystic_chain_playing, on_toggle_mystic_chain);
    connect_button!(rimshot1, MainWindow::set_rimshot1_playing, on_toggle_rimshot1);
    connect_button!(rimshot2, MainWindow::set_rimshot2_playing, on_toggle_rimshot2);
    connect_button!(rimshot3, MainWindow::set_rimshot3_playing, on_toggle_rimshot3);
    connect_button!(rimshot4, MainWindow::set_rimshot4_playing, on_toggle_rimshot4);

    main_window.run()
}
