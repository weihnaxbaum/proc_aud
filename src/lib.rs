pub mod core;
pub mod instruments;

#[cfg(test)]
mod tests;

pub mod prelude {
    pub use crate::{
        core::{Func, Instrument, Note, RenderOutput, RenderedSample, SampleData, Track},
        instruments::{
            CombinedInstruments, FrequencyAdjustedInstrument, Sawtooth, Sine, Square,
            TimbreInstrument, Triangle,
        },
    };
}
