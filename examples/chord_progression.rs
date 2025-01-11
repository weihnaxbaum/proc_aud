//! I–V–vi–IV progression in C major

use std::{sync::Arc, time::Duration};

use proc_aud::prelude::*;

fn main() {
    let mut track = Track::default();

    let timbre = Arc::new(vec![
        (0.25, 0.1),
        (0.5, 0.3),
        (1., 1.),
        (2., 0.5),
        (4., 0.25),
    ]);
    let duration = Duration::from_secs(2);
    let amp = Exp(0.2).f();

    for t in 0..4 {
        let instruments = match t {
            0 => maj_triad("C3", 0),
            1 => maj_triad("G2", 1),
            2 => min_triad("A2", 1),
            3 => maj_triad("F2", 2),
            _ => unreachable!(),
        }
        .map(|hz| {
            (TimbreFunc {
                timbre: Arc::clone(&timbre),
                instrument: Arc::new(Sine::new(hz)),
            }
            .f() + TimbreFunc {
                timbre: Arc::clone(&timbre),
                instrument: Arc::new(Triangle::new(hz)),
            }
            .f())
                * amp.clone()
        });

        track.chord(
            duration * t,
            &[duration],
            &instruments,
            &[(0., 0.5).f(), 0.5.f(), (0.75, 0.5).f()],
        );
    }
    track
        .render(44100.)
        .normalize()
        .save_wav("chord_progression.wav", WavEncoding::I16)
        .expect("Could not write file");
}
