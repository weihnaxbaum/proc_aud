//! Loads the wav file from the `simple` example,
//! adds a note on top of it, and saves it as a new file.

use std::{fs::File, time::Duration};

use proc_aud::prelude::*;

fn main() {
    let simple = RenderOutput::from_wav(
        File::open("simple.wav").expect("File `simple.wav` could not be opened"),
    )
    .expect("`simple.wav` could not be read");
    let note = Note::center(Duration::from_secs(2), Sine::f(110.)).render(44100.);
    (simple + note)
        .normalize()
        .save_wav("load_wav.wav", WavEncoding::I16)
        .expect("Could not write file");
}
