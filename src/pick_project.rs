use eframe::{
    egui,
    egui::{Context, Key, Ui},
    CreationContext, Frame,
};

use crate::project::ProjectManifest;

pub struct PickProject {
    projects: Vec<ProjectManifest>,
}

impl eframe::App for PickProject {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        frame.storage().unwrap().get_string("cuesets");
        egui::CentralPanel::default().show(ctx, |ui| self.ui(ctx, ui));
    }
}

impl PickProject {
    fn ui(&mut self, ctx: &Context, ui: &mut Ui) {}

    pub fn new(cc: &CreationContext) -> Self {
        Self { projects: vec![] }
    }
}
