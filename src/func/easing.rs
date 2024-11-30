use crate::prelude::*;

impl Compute for f32 {
    fn compute(&self, _: ComputeContext) -> f32 {
        *self
    }
}

// TODO: should this really interpolate the values?
impl Compute for (f32, f32) {
    fn compute(&self, context: ComputeContext) -> f32 {
        self.0 + (self.1 - self.0) * context.progress
    }
}

pub struct Exp(pub f32);

impl Compute for Exp {
    fn compute(&self, context: ComputeContext) -> f32 {
        self.0.powf(context.progress)
    }
}

pub struct Pow(pub f32);

impl Compute for Pow {
    fn compute(&self, context: ComputeContext) -> f32 {
        context.progress.powf(self.0)
    }
}
