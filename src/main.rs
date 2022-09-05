use eframe::{
    egui::{
        self,
        plot::{PlotPoint, PlotPoints, Text},
        Label,
    },
    App, AppCreator,
};
use rodio::{
    cpal::platform::AlsaDevices, Device, Devices, OutputDevices, OutputStream, OutputStreamHandle,
    Sink,
};
use std::{
    sync::Arc,
    thread,
    time::{self, Duration},
};

mod piano;

#[macro_use]
extern crate lazy_static;

fn main() {
    // let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Box::new(MyEguiApp::new(cc))),
    );
}

fn play_music() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let notes = ["E5", "E5", "F5", "G5", "G5", "F5", "E5", "D5"];

    for note in notes {
        let sink = Sink::try_new(&stream_handle).unwrap();
        sink.append(piano::PianoWave::new(piano::KEYS[note]));
        thread::sleep(Duration::from_millis(600));
    }
}

fn play_note(key: &str) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    sink.append(piano::PianoWave::new(piano::KEYS[key]));
    thread::sleep(Duration::from_millis(2000));
}

#[derive(Default)]
struct MyEguiApp {
    text: String,
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
            let ted = ui.text_edit_singleline(&mut self.text);

            if ted.changed() {
                let text = self.text.clone();

                thread::spawn(move || match text.chars().last() {
                    Some('q') => play_note("C5"),
                    Some('w') => play_note("D5"),
                    Some('e') => play_note("E5"),
                    Some('r') => play_note("F5"),
                    Some('t') => play_note("G5"),
                    Some('y') => play_note("A5"),
                    Some('u') => play_note("B5"),
                    Some(_) => (),
                    None => (),
                });
                self.text.clear();
            }
        });
    }
}
