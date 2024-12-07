use std::time::Duration;

use proc_aud::prelude::*;

fn main() {
    let mut track = Track::default();
    let duration = Duration::from_secs(1);
    let pan = 0.5.f();
    track.notes.push(
        Note {
            duration,
            instrument: Sine {
                hz: (220., 440.).f(),
            }
            .f(),
            pan: pan.clone(),
        }
        .into(),
    );
    track.notes.push(
        Note {
            duration,
            instrument: Sine { hz: (440., 0.).f() }.f() * (1., 0.).f(),
            pan: pan.clone(),
        }
        .start_at(Duration::from_secs(1)),
    );
    track
        .render(44100.)
        .normalize()
        .save_wav("simple.wav", WavEncoding::I16)
        .expect("Could not write file");
}
