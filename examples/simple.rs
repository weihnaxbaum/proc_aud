use std::time::Duration;

use proc_aud::prelude::*;

fn main() {
    Track::from_cycle(
        &[Duration::from_secs(1)],
        &[Sine::new((220., 440.)).f(), Sine::new((440., 0.)).f()],
        &[0.5.f()],
        2,
    )
    .render(44100.)
    .normalize()
    .save_wav("simple.wav", WavEncoding::I16)
    .expect("Could not write file");
}
