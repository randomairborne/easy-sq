use std::time::Duration;

pub struct Song {
    samples: Vec<f32>,
    sample_rate: usize,
}

impl Song {
    pub fn duration(&self) -> Duration {
        let duration_secs = self.samples.len() as f64 / self.sample_rate as f64;
        Duration::from_secs_f64(duration_secs)
    }
    pub fn from_file() {}
}
