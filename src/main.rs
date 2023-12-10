mod file_load;
mod project;

use rodio::{Decoder, OutputStream, OutputStreamHandle, Source};
use std::io::Cursor;

use eframe::egui;
use eframe::egui::Key;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "EasySQ",
        native_options,
        Box::new(|cc| Box::new(EasySQ::new(cc))),
    )
    .unwrap();
}

struct EasySQ {
    _useless_output_stream: OutputStream,
    output: OutputStreamHandle,
}

impl EasySQ {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        Self {
            _useless_output_stream: stream,
            output: stream_handle,
        }
    }
}

impl eframe::App for EasySQ {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Play the best song in cats by pressing space");
            if ctx.input(|i| i.key_pressed(Key::Space)) {
                self.play("skimble.wav");
            }
        });
    }
}

impl EasySQ {
    fn play(&self, file_name: &str) {
        let file_data = std::fs::read(file_name).unwrap();
        let file_cursor = Cursor::new(file_data);
        let decoded = Decoder::new(file_cursor).unwrap().amplify(1.0);
        self.output.play_raw(decoded.convert_samples()).unwrap();
        println!("Started {file_name}");
    }
}
