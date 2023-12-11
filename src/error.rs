use rodio::cpal::DeviceNameError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Rodio PlayError: {0}")]
    Play(#[from] rodio::PlayError),
    #[error("Rodio DevicesError: {0}")]
    Devices(#[from] rodio::DevicesError),
    #[error("Rodio DeviceNameError: {0}")]
    DeviceName(#[from] DeviceNameError),
    #[error("Rodio StreamError: {0}")]
    Stream(#[from] rodio::StreamError),
}

pub enum CpalError {}
