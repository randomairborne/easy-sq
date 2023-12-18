use std::{collections::HashMap, path::Path};

use serde::{Deserialize, Serialize};

use crate::error::Error;
pub struct Project {
    pub manifest: ProjectManifest,
    pub loaded_tracks: HashMap<String, Vec<f32>>,
}

impl Project {
    pub(crate) fn load(path: impl AsRef<Path>) -> Result<Self, Error> {
        let path = path.as_ref();
        let meta_path = path.join("metadata.json");
        println!("{}", meta_path.display());
        let meta_file = std::fs::OpenOptions::new().read(true).open(meta_path)?;
        let manifest: ProjectManifest = serde_json::from_reader(meta_file)?;
        let mut loaded_tracks = HashMap::new();
        for track in &manifest.tracks {
            loaded_tracks.insert(track.name.clone(), vec![]);
        }
        Ok(Self {
            manifest,
            loaded_tracks,
        })
    }
}

#[derive(Deserialize, Serialize)]
pub struct ProjectManifest {
    pub tracks: Vec<TrackManifest>,
    pub metadata: ProjectMetadata,
}

#[derive(Deserialize, Serialize)]
pub struct TrackManifest {
    pub id: String,
    pub name: String,
    pub duration: usize,
}

#[derive(Deserialize, Serialize)]
pub struct ProjectMetadata {
    pub author: String,
    pub title: String,
}
