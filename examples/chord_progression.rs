//! I–V–vi–IV progression in C major

use std::{rc::Rc, time::Duration};

use proc_aud::prelude::*;

fn main() {
    let mut track = Track::default();

    let timbre = Rc::new(vec![
        (0.25, 0.1),
        (0.5, 0.3),
        (1., 1.),
        (2., 0.5),
        (4., 0.25),
    ]);
    let duration = Duration::from_secs(2);
    let amp = Exp(0.2).f();

    for t in 0..4 {
        let hz_values = match t {
            0 => [tet("C3"), tet("E3"), tet("G3")],
            1 => [tet("B2"), tet("D3"), tet("G3")],
            2 => [tet("C3"), tet("E3"), tet("A3")],
            3 => [tet("C3"), tet("F3"), tet("A3")],
            _ => unreachable!(),
        };
        for (i, hz) in hz_values.iter().enumerate() {
            let instrument = (TimbreFunc {
                timbre: Rc::clone(&timbre),
                instrument: Rc::new(Sine { hz: hz.f() }),
            }
            .f() + TimbreFunc {
                timbre: Rc::clone(&timbre),
                instrument: Rc::new(Triangle { hz: hz.f() }),
            }
            .f())
                * amp.clone();

            let starting_pan = i as f32 / 2.;
            track.notes.push(
                Note {
                    duration,
                    instrument,
                    pan: (starting_pan, 0.5).f(),
                }
                .start_at(duration * t),
            );
        }
    }
    track
        .render(44100.)
        .normalize()
        .save_wav("chord_progression.wav", WavEncoding::I16)
        .expect("Could not write file");
}
