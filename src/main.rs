mod sound;
mod gui;

#[macro_use]
extern crate lazy_static;

fn main() {
    // let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Box::new(gui::MyEguiApp::new(cc))),
    );
}
