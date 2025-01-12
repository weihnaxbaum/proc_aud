use std::time::Duration;

use proc_aud::prelude::*;

fn main() {
    Track::from_cycle(
        &[Duration::from_secs(1)],
        &[Sine::f((220., 440.)), Sine::f((440., 0.))],
        &[0.5.f()],
        2,
    )
    .render(44100.)
    .normalize()
    .save_wav("simple.wav", WavEncoding::I16)
    .expect("Could not write file");
}
