use std::io::Cursor;

use cpal::Device;
use eframe::{
    egui,
    egui::{Color32, Frame, Grid, Key, Style},
};
use egui_extras::{Column, TableBuilder};

use crate::{player::Player, project::Project};

pub struct EasySQ {
    player: Player,
    project: Project,
    selected_idx: usize,
}

impl EasySQ {
    pub(crate) fn new(cc: &eframe::CreationContext<'_>, project: Project) -> Self {
        Self {
            player: Player::new().unwrap(),
            project,
            selected_idx: 0,
        }
    }
}

const SELECTED: Color32 = Color32::from_rgb(0, 0, 255);

impl eframe::App for EasySQ {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            Grid::new("main_cue_list").striped(true).show(ui, |ui| {
                ui.label("Cue");
                ui.label("Track");
                ui.label("Duration");
                ui.end_row();
                for (index, track) in self.project.manifest.tracks.iter().enumerate() {
                    let mut frame = Frame::none();
                    if index == self.selected_idx {
                        frame.fill = SELECTED;
                    }
                    frame.show(ui, |ui| {
                        ui.label(&track.id);
                    });
                    ui.label(&track.name);
                    ui.label(track.duration.to_string());
                    ui.end_row();
                }
            });
            if ctx.input(|i| i.key_pressed(Key::Space)) {
                self.selected_idx += 1;
            }
        });
    }
}

impl EasySQ {
    fn play(&self, file_name: &str) {
        println!("Started {file_name}");
    }
}
