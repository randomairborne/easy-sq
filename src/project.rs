use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct Project {
    manifest: ProjectManifest,
    loaded_tracks: HashMap<String, Vec<f32>>,
}

#[derive(Deserialize, Serialize)]
pub struct ProjectManifest {
    tracks: Vec<TrackManifest>,
    metadata: ProjectMetadata,
}

#[derive(Deserialize, Serialize)]
pub struct TrackManifest {
    id: String,
    name: String,
    duration: usize
}

#[derive(Deserialize, Serialize)]
pub struct ProjectMetadata {
    author: String,
    title: String,
}
