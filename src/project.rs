use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use crate::error::Error;

use std::path::Path;
pub struct Project {
    manifest: ProjectManifest,
    loaded_tracks: HashMap<String, Vec<f32>>,
}

impl Project {
    pub(crate) fn load(path: impl AsRef<Path>) -> Result<Self, Error> {
        let path = path.as_ref();
        let meta_file = std::fs::OpenOptions::new().open(path.join("metadata.json"))?;
        let manifest: ProjectManifest = serde_json::from_reader(meta_file)?;
        for track in manifest.tracks {
            track.
        }
        Self {
            manifest,
            loaded_tracks
        }
    }
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
    duration: usize,
}

#[derive(Deserialize, Serialize)]
pub struct ProjectMetadata {
    author: String,
    title: String,
}
