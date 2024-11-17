use std::{f32::consts::TAU, time::Duration};

use proc_aud::*;

struct SineWave;

impl Instrument for SineWave {
    fn sample(&self, data: SampleData) -> f32 {
        (TAU * data.note_time_elapsed().as_secs_f32() * data.hz()).sin()
    }
}

fn main() {
    let mut track = Track::default();
    let duration = Duration::from_secs(1);
    let instrument = &SineWave;
    track.notes.push(Note {
        start: Duration::ZERO,
        duration,
        instrument,
        hz: &|x| 220. * x + 220.,
        amplitude: &|_| 1.,
        pan: &|_| 0.5,
    });
    track.notes.push(Note {
        start: Duration::from_secs(1),
        duration,
        instrument,
        hz: &|x| -440. * x + 440.,
        amplitude: &|x| 1. - x,
        pan: &|_| 0.5,
    });
    track
        .save_wav("simple.wav", 44100., 32)
        .expect("Could not write file");
}
