#![windows_subsystem = "windows"]

use crate::{project::Project, ui::EasySQ};

mod error;
mod file_load;
mod player;
mod project;
mod song;
mod ui;

fn main() {
    let native_options = eframe::NativeOptions::default();
    let project_path = std::env::args().nth(1).unwrap();
    let project = Project::load(project_path).unwrap();
    eframe::run_native(
        "EasySQ",
        native_options,
        Box::new(|cc| Box::new(EasySQ::new(cc, project))),
    )
    .unwrap();
}
