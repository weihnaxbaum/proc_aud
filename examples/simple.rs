use std::{rc::Rc, time::Duration};

use proc_aud::prelude::*;

fn main() {
    let mut track = Track::default();
    let duration = Duration::from_secs(1);
    let pan: Func = Rc::new(0.5);
    track.notes.push(Note {
        start: Duration::ZERO,
        duration,
        instrument: Rc::new(Sine {
            hz: Rc::new((220., 440.)),
        }),
        amp: Rc::new(1.),
        pan: Rc::clone(&pan),
    });
    track.notes.push(Note {
        start: Duration::from_secs(1),
        duration,
        instrument: Rc::new(Sine {
            hz: Rc::new((440., 0.)),
        }),
        amp: Rc::new((1., 0.)),
        pan: Rc::clone(&pan),
    });
    track
        .render(44100.)
        .normalize()
        .save_wav("simple.wav", WavEncoding::I16)
        .expect("Could not write file");
}
