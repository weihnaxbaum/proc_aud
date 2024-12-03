use std::rc::Rc;

use crate::{func::periodic::PeriodicCompute, prelude::*};

pub struct CombinedFuncs(pub Vec<Func>);

impl Compute for CombinedFuncs {
    fn compute(&self, context: ComputeContext) -> f32 {
        self.0.iter().map(|i| i.compute(context)).sum()
    }
}

pub struct TimbreFunc {
    pub timbre: Rc<Vec<(f32, f32)>>,
    pub instrument: Rc<dyn PeriodicCompute>,
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
