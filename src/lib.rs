pub mod core;
pub mod freq;
pub mod instruments;

#[cfg(test)]
mod tests;

pub mod prelude {
    pub use crate::{
        core::{Func, Instrument, Note, RenderOutput, RenderedSample, SampleData, Track},
        freq::tet,
        instruments::{
            CombinedInstruments, FrequencyAdjustedInstrument, Sawtooth, Sine, Square,
            TimbreInstrument, Triangle,
        },
    };
}
