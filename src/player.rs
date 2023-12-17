use rodio::{cpal::traits::HostTrait, DeviceTrait, Devices, OutputStream, OutputStreamHandle};

use crate::{error::Error, song::Song};

pub struct Player {
    _useless_output_stream: OutputStream,
    output: OutputStreamHandle,
}

impl Player {
    pub fn new() -> Result<Self, Error> {
        let (stream, stream_handle) = OutputStream::try_default()?;
        let host = rodio::cpal::default_host();
        let devices = host.devices()?;
        for device in devices {
            let name = device.name()?;
            println!("{name}");
        }
        Ok(Self {
            _useless_output_stream: stream,
            output: stream_handle,
        })
    }

    pub fn pause(&mut self) {}

    pub fn play(&mut self) {}

    pub fn set_song(&mut self, song: Song) {}
}
