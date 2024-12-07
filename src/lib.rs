pub mod freq;
pub mod func;
pub mod note;
pub mod render;
pub mod track;
#[cfg(feature = "wav")]
pub mod wav;

#[cfg(test)]
mod tests;

pub mod prelude {
    #[cfg(feature = "noise")]
    pub use crate::func::noise::WhiteNoise;
    #[cfg(feature = "wav")]
    pub use crate::wav::WavEncoding;
    pub use crate::{
        freq::tet,
        func::{
            easing::{Exp, Pow},
            modifier::TimbreFunc,
            periodic::{Sawtooth, Sine, Square, Triangle},
            Compute, ComputeContext, Func, IntoFunc,
        },
        note::Note,
        render::{RenderOutput, RenderedSample},
        track::Track,
    };
}
