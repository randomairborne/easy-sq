#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Rodio PlayError: {0}")]
    Play(#[from] rodio::PlayError),
    #[error("Rodio DevicesError: {0}")]
    Devices(#[from] rodio::DevicesError),
    #[error("Rodio DeviceNameError: {0}")]
    DeviceName(#[from] rodio::cpal::DeviceNameError),
    #[error("Rodio StreamError: {0}")]
    Stream(#[from] rodio::StreamError),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

pub enum CpalError {}
