use crate::prelude::*;

#[derive(Clone)]
pub struct WhiteNoise;

impl Compute for WhiteNoise {
    fn compute(&self, _: ComputeContext) -> f32 {
        fastrand::f32()
    }
}
