use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Data, FromSample, Sample, SampleFormat,
};

fn write_silence<T: Sample>(data: &mut [T], _: &cpal::OutputCallbackInfo) {
    for sample in data.iter_mut() {
        *sample = Sample::EQUILIBRIUM;
    }
}
use crate::{error::Error, song::Song};

pub struct Player {}

impl Player {
    pub fn new() -> Result<Self, Error> {
        let host = cpal::default_host();
        let output = host.default_output_device().ok_or(Error::NoOutputDevice)?;
        Ok(Self {})
    }

    pub fn pause(&mut self) {}

    pub fn play(&mut self) {}

    pub fn set_song(&mut self, song: Song) {}
}
