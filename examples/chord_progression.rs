//! I–V–vi–IV progression in C major

use std::{rc::Rc, time::Duration};

use proc_aud::prelude::*;

const C3: f32 = 130.8128;
const E3: f32 = 164.8138;
const G3: f32 = 195.9977;

const B2: f32 = 123.4708;
const D3: f32 = 146.8324;
// G3

// C3
// E3
const A3: f32 = 220.;

// C3
const F3: f32 = 174.6141;
// A3

fn main() {
    let mut track = Track::default();
    let instrument = TimbreInstrument {
        timbre: vec![(0.25, 0.1), (0.5, 0.3), (1., 1.), (2., 0.5), (4., 0.25)],
        instrument: &CombinedInstruments {
            instruments: vec![&Sine, &Triangle],
        },
    };
    let duration = Duration::from_secs(2);
    let amplitude: Func = Rc::new(|x| 0.2f32.powf(x));
    for t in 0..4 {
        let hz_values = match t {
            0 => [C3, E3, G3],
            1 => [B2, D3, G3],
            2 => [C3, E3, A3],
            3 => [C3, F3, A3],
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
                amplitude: Rc::clone(&amplitude),
                pan: Rc::new(move |x| starting_pan + (0.5 - starting_pan) * x),
            });
        }
    }
    track
        .save_wav("chord_progression.wav", 44100., 32)
        .expect("Could not write file");
}
