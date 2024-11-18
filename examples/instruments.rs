use std::{rc::Rc, time::Duration};

use proc_aud::prelude::*;

fn main() {
    let mut track = Track::default();
    let instruments: [&dyn Instrument; 6] = [
        &Sine,
        &Square,
        &Triangle,
        &Sawtooth,
        &CombinedInstruments {
            instruments: vec![&Sine, &Square, &Triangle, &Sawtooth],
        },
        &TimbreInstrument {
            timbre: vec![(0.5, 0.3), (1., 1.), (2., 0.5)],
            instrument: &Sine,
        },
    ];
    let hz: Func = Rc::new(|_| 220.);
    let amp: Func = Rc::new(|x| 1.5 - x);
    let pan: Func = Rc::new(|_| 0.5);
    for (i, instrument) in instruments.into_iter().enumerate() {
        track.notes.push(Note {
            start: Duration::from_secs(i as u64 * 3),
            duration: Duration::from_secs(2),
            instrument,
            hz: Rc::clone(&hz),
            amp: Rc::clone(&amp),
            pan: Rc::clone(&pan),
        });
    }
    track
        .render(44100.)
        .save_wav("instruments.wav", 32)
        .expect("Could not write file");
}
