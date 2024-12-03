use std::{rc::Rc, time::Duration};

use proc_aud::prelude::*;

fn main() {
    let mut track = Track::default();

    let hz = 220.0.f();
    let sine = Sine { hz: hz.clone() }.f();
    let square = Square { hz: hz.clone() }.f();
    let triangle = Triangle { hz: hz.clone() }.f();
    let sawtooth = Sawtooth { hz: hz.clone() }.f();
    let white_noise = WhiteNoise.f();

    let instruments: [Func; 7] = [
        sine.clone(),
        square.clone(),
        triangle.clone(),
        sawtooth.clone(),
        white_noise.clone(),
        sine + square + triangle + sawtooth + white_noise,
        TimbreFunc {
            timbre: Rc::new(vec![(0.5, 0.3), (1., 1.), (2., 0.5)]),
            instrument: Rc::new(Sine { hz }),
        }
        .f(),
    ];
    let amp = (1.5, 0.5).f();
    let pan = 0.5.f();
    for (i, instrument) in instruments.into_iter().enumerate() {
        track.notes.push(Note {
            start: Duration::from_secs(i as u64 * 3),
            duration: Duration::from_secs(2),
            instrument: instrument * amp.clone(),
            pan: pan.clone(),
        });
    }
    track
        .render(44100.)
        .normalize()
        .save_wav("instruments.wav", WavEncoding::I16)
        .expect("Could not write file");
}
