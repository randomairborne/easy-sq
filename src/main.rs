#![windows_subsystem = "windows"]

mod error;
mod file_load;
mod pick_project;
mod player;
mod project;
mod song;
mod ui;

use crate::pick_project::PickProject;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "EasySQ",
        native_options,
        Box::new(|cc| Box::new(PickProject::new(cc))),
    )
    .unwrap();
}
