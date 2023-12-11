#![windows_subsystem = "windows"]

mod error;
mod file_load;
mod player;
mod project;
mod song;

use std::io::Cursor;

use crate::player::Player;
use eframe::{egui, egui::Key};
use rodio::{Decoder, OutputStream, OutputStreamHandle, Source};

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
    player: Player,
}

impl EasySQ {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            player: Player::new().unwrap(),
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
        println!("Started {file_name}");
    }
}
