use std::time::Duration;

use proc_aud::prelude::*;

fn main() {
    let mut track = Track::default();
    let duration = Duration::from_secs(1);
    let instrument = &Sine;
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
