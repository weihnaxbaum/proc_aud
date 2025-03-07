use std::time::Duration;

use proc_aud::prelude::*;

fn main() {
    Gen {
        instrument: Sine::f(440.0) * Exp(0.5).f() - 0.5.f(),
        note_duration: Duration::from_secs_f32(0.5),
        min_semitone_shift: -24.0,
        max_semitone_shift: 24.0,
        seed: 0,
    }
    .gen(Duration::from_secs(4))
    .render(44100.0)
    .normalize()
    .save_wav("gen.wav", WavEncoding::I16)
    .expect("Could not write to file");
}
