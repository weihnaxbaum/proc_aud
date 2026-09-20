//! A snippet from "Canon in D" by Johann Pachelbel.
//!
//! [Source](https://cascademethod.com/wp-content/uploads/2018/11/Canon_in_D.pdf) (starting at bar 29)

use std::{sync::Arc, time::Duration};

use proc_aud::prelude::*;

const BAR_SECS: f32 = 2.5;
const DEEP_PAN: f32 = 0.4;
const HIGH_PAN: f32 = 0.6;

fn main() {
    let mut track = Track::default();

    melody(
        &mut track,
        &[
            // Bar 1
            "A5", "F#5", "G5", "A5", "F#5", "G5", "A5", "A4", "B4", "C#5", "D5", "E5", "F#5", "G5",
            // Bar 2
            "F#5", "D5", "E5", "F#5", "F#4", "G4", "A4", "B4", "A4", "G4", "A4", "F#4", "G4", "B4",
            // Bar 3
            "G4", "B4", "A4", "G4", "F#4", "E4", "F#4", "E4", "D4", "E4", "F#4", "G4", "A4", "B4",
            // Bar 4
            "G4", "B4", "A4", "B4", "A4", "G4", "A4", "B4", "C#5", "D5", "E5", "F#5", "G5", "A5",
            // Bar 5
            "F#5", "D5", "E5", "F#5", "E5", "D5", "E5", "C#5", "D5", "E5", "F#5", "E5", "D5", "C#5",
            // Bar 6
            "D5", "B4", "C#5", "D5", "D4", "E4", "F#4", "G4", "F#4", "E4", "F#4", "D5", "C#5", "D5",
            // Bar 7
            "B4", "D5", "C#5", "B4", "A4", "G4", "A4", "G4", "F#4", "G4", "A4", "B4", "C#5", "D5",
            // Bar 8
            "B4", "D5", "C#5", "D5", "C#5", "B4", "C#5", "D5", "E5", "D5", "C#5", "D5", "B4", "C#5",
        ],
    );
    bass(
        &mut track,
        &[
            "D3", "A3", "D4", "A3", "A2", "E3", "A3", "E3", // Bar 1
            "B2", "F#3", "B3", "F#3", "F#2", "C#3", "F#3", "C#3", // Bar 2
            "G2", "D2", "G3", "B3", "D2", "A2", "D3", "A2", // Bar 3
            "G2", "D2", "G3", "D2", "A2", "E3", "A3", "E3", // Bar 4
        ]
        .repeat(2),
    );

    track
        .render(44100.0)
        .normalize()
        .save_wav("canon_in_d.wav", WavEncoding::I16)
        .expect("Could not write file");
}

fn melody(track: &mut Track, notes: &[&str]) {
    let timbre = Arc::new(vec![
        (0.25, 0.1),
        (0.5, 0.3),
        (1.0, 1.0),
        (2.0, 0.5),
        (4.0, 0.25),
        (8.0, 0.1),
    ]);

    let vec: Vec<_> = notes
        .iter()
        .enumerate()
        .map(|(i, note)| {
            (
                *note,
                1.0 / if i % 14 == 0 || i % 14 == 3 {
                    8.0
                } else {
                    16.0
                },
            )
        })
        .collect();

    sequence(track, &vec, HIGH_PAN, timbre);
}

fn bass(track: &mut Track, notes: &[&str]) {
    let timbre = Arc::new(vec![
        (0.25, 0.2),
        (0.5, 0.5),
        (1., 1.),
        (2., 0.5),
        (4., 0.25),
    ]);

    let vec: Vec<_> = notes.iter().map(|note| (*note, 1.0 / 8.0)).collect();

    sequence(track, &vec, DEEP_PAN, timbre);
}

fn sequence(track: &mut Track, notes: &[(&str, f32)], pan: f32, timbre: Arc<Vec<(f32, f32)>>) {
    let mut start = Duration::ZERO;
    for (name, dur_rel) in notes {
        let instrument = TimbreFunc {
            timbre: timbre.clone(),
            instrument: Arc::new(Sine::new(tet(name))),
        };
        let dur = Duration::from_secs_f32(BAR_SECS * dur_rel);
        let note = Note::new(dur, instrument.f() * Exp(0.5).f(), pan);
        track.notes.push(note.start_at(start));
        start += dur;
    }
}
