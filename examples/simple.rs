use std::{rc::Rc, time::Duration};

use proc_aud::prelude::*;

fn main() {
    let mut track = Track::default();
    let duration = Duration::from_secs(1);
    let instrument = &Sine;
    let pan: Func = Rc::new(|_| 0.5);
    track.notes.push(Note {
        start: Duration::ZERO,
        duration,
        instrument,
        hz: Rc::new(|x| 220. * x + 220.),
        amplitude: Rc::new(|_| 1.),
        pan: Rc::clone(&pan),
    });
    track.notes.push(Note {
        start: Duration::from_secs(1),
        duration,
        instrument,
        hz: Rc::new(|x| -440. * x + 440.),
        amplitude: Rc::new(|x| 1. - x),
        pan: Rc::clone(&pan),
    });
    track
        .save_wav("simple.wav", 44100., 32)
        .expect("Could not write file");
}
