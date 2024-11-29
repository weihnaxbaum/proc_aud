use std::{rc::Rc, time::Duration};

use proc_aud::prelude::*;

fn main() {
    let mut track = Track::default();

    let hz: Func = Rc::new(220.);
    let sine: Func = Rc::new(Sine { hz: Rc::clone(&hz) });
    let square: Func = Rc::new(Square { hz: Rc::clone(&hz) });
    let triangle: Func = Rc::new(Triangle { hz: Rc::clone(&hz) });
    let sawtooth: Func = Rc::new(Sawtooth { hz: Rc::clone(&hz) });
    let white_noise: Func = Rc::new(WhiteNoise);

    let instruments: [Func; 7] = [
        Rc::clone(&sine),
        Rc::clone(&square),
        Rc::clone(&triangle),
        Rc::clone(&sawtooth),
        Rc::clone(&white_noise),
        Rc::new(CombinedFuncs(vec![
            sine,
            square,
            triangle,
            sawtooth,
            white_noise,
        ])),
        Rc::new(TimbreFunc {
            timbre: Rc::new(vec![(0.5, 0.3), (1., 1.), (2., 0.5)]),
            instrument: Rc::new(Sine { hz }),
        }),
    ];
    let amp: Func = Rc::new((1.5, 0.5));
    let pan: Func = Rc::new(0.5);
    for (i, instrument) in instruments.into_iter().enumerate() {
        track.notes.push(Note {
            start: Duration::from_secs(i as u64 * 3),
            duration: Duration::from_secs(2),
            instrument,
            amp: Rc::clone(&amp),
            pan: Rc::clone(&pan),
        });
    }
    track
        .render(44100.)
        .normalize()
        .save_wav("instruments.wav", WavEncoding::I16)
        .expect("Could not write file");
}
