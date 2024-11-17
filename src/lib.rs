pub mod core;

#[cfg(test)]
mod tests;

pub mod prelude {
    pub use crate::core::{Instrument, Note, RenderOutput, RenderedSample, SampleData, Track};
}
