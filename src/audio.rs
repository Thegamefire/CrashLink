use std::io::Cursor;
use rodio::{Decoder, MixerDeviceSink};
use crate::SOUND;

pub struct AudioManager {
    sink_handle: MixerDeviceSink
}
impl AudioManager {
    pub fn new() -> Self {
        AudioManager {
            sink_handle: rodio::DeviceSinkBuilder::open_default_sink()
                .expect("No Audio Device Found")
        }
    }
    pub fn play_sound(&self) {
        let source = Decoder::try_from(Cursor::new(SOUND)).unwrap();
        self.sink_handle.mixer().add(source);
    }
}