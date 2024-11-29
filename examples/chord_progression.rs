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
    let amp: Func = Rc::new(|context: ComputeContext| 0.2f32.powf(context.progress));

    for t in 0..4 {
        let hz_values = match t {
            0 => [tet("C3"), tet("E3"), tet("G3")],
            1 => [tet("B2"), tet("D3"), tet("G3")],
            2 => [tet("C3"), tet("E3"), tet("A3")],
            3 => [tet("C3"), tet("F3"), tet("A3")],
            _ => unreachable!(),
        };
        for (i, hz) in hz_values.iter().enumerate() {
            let instrument = Rc::new(CombinedFuncs(vec![
                Rc::new(TimbreFunc {
                    timbre: Rc::clone(&timbre),
                    instrument: Rc::new(Sine { hz: Rc::new(*hz) }),
                }),
                Rc::new(TimbreFunc {
                    timbre: Rc::clone(&timbre),
                    instrument: Rc::new(Triangle { hz: Rc::new(*hz) }),
                }),
            ]));

            let starting_pan = i as f32 / 2.;
            track.notes.push(Note {
                start: duration * t,
                duration,
                instrument,
                amp: Rc::clone(&amp),
                pan: Rc::new((starting_pan, 0.5)),
            });
        }
    }
    track
        .render(44100.)
        .normalize()
        .save_wav("chord_progression.wav", WavEncoding::I16)
        .expect("Could not write file");
}
