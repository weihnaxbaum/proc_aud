//! I–V–vi–IV progression in C major

use std::{rc::Rc, time::Duration};

use proc_aud::prelude::*;

fn main() {
    let mut track = Track::default();
    let instrument = TimbreInstrument {
        timbre: vec![(0.25, 0.1), (0.5, 0.3), (1., 1.), (2., 0.5), (4., 0.25)],
        instrument: &CombinedInstruments {
            instruments: vec![&Sine, &Triangle],
        },
    };
    let duration = Duration::from_secs(2);
    let amp: Func = Rc::new(|x| 0.2f32.powf(x));
    for t in 0..4 {
        let hz_values = match t {
            0 => [tet("C3"), tet("E3"), tet("G32")],
            1 => [tet("B2"), tet("D3"), tet("G3")],
            2 => [tet("C3"), tet("E3"), tet("A3")],
            3 => [tet("C3"), tet("F3"), tet("A3")],
            _ => unreachable!(),
        };
        for (i, hz) in hz_values.iter().enumerate() {
            let hz = *hz;
            let starting_pan = i as f32 / 2.;
            track.notes.push(Note {
                start: duration * t,
                duration,
                instrument: &instrument,
                hz: Rc::new(move |_| hz),
                amp: Rc::clone(&amp),
                pan: Rc::new(move |x| starting_pan + (0.5 - starting_pan) * x),
            });
        }
    }
    track
        .render(44100.)
        .save_wav("chord_progression.wav", 32)
        .expect("Could not write file");
}
