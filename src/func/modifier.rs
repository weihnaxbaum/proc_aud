use std::sync::Arc;

use crate::{func::periodic::PeriodicCompute, prelude::*};

pub struct TimbreFunc {
    pub timbre: Arc<Vec<(f32, f32)>>,
    pub instrument: Arc<dyn PeriodicCompute>,
}

impl Compute for TimbreFunc {
    fn compute(&self, context: ComputeContext) -> f32 {
        let hz = self.instrument.hz().compute(context);
        self.timbre
            .iter()
            .map(|(multiplier, amplitude)| {
                self.instrument.compute_with_hz(context, hz * multiplier) * amplitude
            })
            .sum()
    }
}
