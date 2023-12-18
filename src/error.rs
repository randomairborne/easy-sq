#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("CPAL DeviceNameError: {0}")]
    DeviceName(#[from] cpal::DeviceNameError),
    #[error("CPAL DevicesNameError: {0}")]
    Devices(#[from] cpal::DevicesError),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("No output device found!")]
    NoOutputDevice,
}
