use std::{sync::Arc, time::Duration};

use proc_aud::prelude::*;

fn main() {
    let mut track = Track::default();

    let hz = 220.;
    let sine = Sine::f(hz);
    let square = Square::f(hz);
    let triangle = Triangle::f(hz);
    let sawtooth = Sawtooth::f(hz);
    let white_noise = WhiteNoise.f();

    let instruments: [Func; 7] = [
        sine.clone(),
        square.clone(),
        triangle.clone(),
        sawtooth.clone(),
        white_noise.clone(),
        sine + square + triangle + sawtooth + white_noise,
        TimbreFunc {
            timbre: Arc::new(vec![(0.5, 0.3), (1., 1.), (2., 0.5)]),
            instrument: Arc::new(Sine::new(hz)),
        }
        .f(),
    ];
    let amp = (1.5, 0.5).f();
    for (i, instrument) in instruments.into_iter().enumerate() {
        track.notes.push(
            Note::center(Duration::from_secs(2), instrument * amp.clone())
                .start_at(Duration::from_secs(i as u64 * 3)),
        );
    }
    track
        .render(44100.)
        .normalize()
        .save_wav("instruments.wav", WavEncoding::I16)
        .expect("Could not write file");
}
