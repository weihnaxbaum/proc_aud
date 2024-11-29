pub mod core;
pub mod freq;
pub mod func;

#[cfg(test)]
mod tests;

pub mod prelude {
    pub use crate::{
        core::{Compute, ComputeContext, Func, Note, RenderOutput, RenderedSample, Track},
        freq::tet,
        func::{
            modifier::{CombinedFuncs, TimbreFunc},
            periodic::{Sawtooth, Sine, Square, Triangle},
        },
    };
}
