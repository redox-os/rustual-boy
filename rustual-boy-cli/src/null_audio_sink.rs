#![allow(dead_code)]

use rustual_boy_core::sinks::{AudioFrame, SinkRef};

pub struct NullAudioSink { }

impl SinkRef<[AudioFrame]> for NullAudioSink {
    fn append(&mut self, buffer: &[(i16, i16)]) {
    }
}
