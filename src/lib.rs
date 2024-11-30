pub mod core;
pub mod freq;
pub mod func;

#[cfg(test)]
mod tests;

pub mod prelude {
    #[cfg(feature = "wav")]
    pub use crate::core::WavEncoding;
    #[cfg(feature = "noise")]
    pub use crate::func::noise::WhiteNoise;
    pub use crate::{
        core::{Compute, ComputeContext, Func, Note, RenderOutput, RenderedSample, Track},
        freq::tet,
        func::{
            easing::{Exp, Pow},
            modifier::{CombinedFuncs, TimbreFunc},
            periodic::{Sawtooth, Sine, Square, Triangle},
        },
    };
}
