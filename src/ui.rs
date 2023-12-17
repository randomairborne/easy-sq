use std::io::Cursor;

use eframe::{egui, egui::Key};
use rodio::Decoder;

use crate::{player::Player, project::Project};

struct EasySQ {
    player: Player,
    project: Project,
}

impl EasySQ {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            player: Player::new().unwrap(),
            project: todo!(),
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
        let decoded = Decoder::new(file_cursor).unwrap();
        println!("Started {file_name}");
    }
}
