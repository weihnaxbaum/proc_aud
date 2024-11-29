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
