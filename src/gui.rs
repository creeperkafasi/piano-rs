use std::thread;

use eframe::egui::{
    self,
    plot::{Line, Plot, PlotPoints},
};

use crate::sound::{piano, play_note};

fn match_keys(text: &str) -> &str {
    match text.chars().last() {
        Some('q') => "C5",
        Some('w') => "D5",
        Some('e') => "E5",
        Some('r') => "F5",
        Some('t') => "G5",
        Some('y') => "A5",
        Some('u') => "B5",
        Some(_) => "",
        None => "",
    }
}

#[derive(Default)]
pub struct MyEguiApp {
    text: String,
    wave_points: Vec<[f64; 2]>,
}

impl MyEguiApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut s = Self::default();
        s.wave_points = vec![[0.0, 0.0]];
        return s;
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Enter characters q,w,e,r,t,y,u to play keys from C5 (Do) to B5 (Si)");
            // RichText::
            let ted = ui.text_edit_singleline(&mut self.text);

            let wp = self.wave_points.to_vec();

            let line: Line = Line::new(PlotPoints::new(wp));

            Plot::new("wave")
                .view_aspect(2.0)
                .data_aspect(1.0)
                .show(ui, |plot_ui| plot_ui.line(line));

            if ted.changed() {
                let text = self.text.clone();

                if match_keys(&self.text) != "" {
                    self.wave_points = (0..3000)
                        .map(|i| {
                            let x = i as f32 * 0.001;
                            [
                                x as f64,
                                piano::PianoWave::wave_function(
                                    piano::KEYS[match_keys(&text)] / 100.,
                                    x,
                                    10,
                                ) as f64,
                            ]
                        })
                        .collect::<Vec<[f64; 2]>>();

                    // self.wavePoints = (0..2000)
                    //     .map(|i| {
                    //         let x = i as f64 * 0.01;
                    //         [
                    //         PlotPoint::new(
                    //             x,
                    //             piano::PianoWave::wave_function(
                    //                 piano::KEYS[match_keys(&self.text)],
                    //                 x as f32,
                    //                 5,
                    //             ).into()
                    //             )
                    //         ]
                    //     }).collect::<Vec<PlotPoint>>();
                }

                thread::spawn(move || play_note(match_keys(&text.clone())));
                self.text.clear();
            }
        });
    }
}
