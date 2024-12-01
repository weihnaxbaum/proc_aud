use std::f32::consts::{PI, TAU};

use crate::prelude::*;

// TODO: implement `Compute` for all `PeriodicCompute` implementors:
// Possibly difficult due to conflicting trait implementations
pub trait PeriodicCompute {
    fn hz(&self) -> Func;

    fn compute_with_hz(&self, context: ComputeContext, hz: Func) -> f32;

    fn compute_with_hz_multiplier(&self, context: ComputeContext, multiplier: f32) -> f32 {
        let hz = self.hz();
        let new_hz = (move |context| hz.compute(context) * multiplier).f();
        self.compute_with_hz(context, new_hz)
    }
}

#[derive(Clone)]
pub struct Sine {
    pub hz: Func,
}

impl PeriodicCompute for Sine {
    fn hz(&self) -> Func {
        self.hz.clone()
    }
    fn compute_with_hz(&self, context: ComputeContext, hz: Func) -> f32 {
        (TAU * context.time_elapsed.as_secs_f32() * hz.compute(context)).sin()
    }
}

impl Compute for Sine {
    fn compute(&self, context: ComputeContext) -> f32 {
        self.compute_with_hz(context, self.hz())
    }
}

#[derive(Clone)]
pub struct Square {
    pub hz: Func,
}

impl PeriodicCompute for Square {
    fn hz(&self) -> Func {
        self.hz.clone()
    }
    fn compute_with_hz(&self, context: ComputeContext, hz: Func) -> f32 {
        let phase = (TAU * context.time_elapsed.as_secs_f32() * hz.compute(context)) % TAU;
        if phase < PI {
            1.0
        } else {
            -1.0
        }
    }
}

impl Compute for Square {
    fn compute(&self, context: ComputeContext) -> f32 {
        self.compute_with_hz(context, self.hz())
    }
}

#[derive(Clone)]
pub struct Triangle {
    pub hz: Func,
}

impl PeriodicCompute for Triangle {
    fn hz(&self) -> Func {
        self.hz.clone()
    }
    fn compute_with_hz(&self, context: ComputeContext, hz: Func) -> f32 {
        let phase = (TAU * context.time_elapsed.as_secs_f32() * hz.compute(context)) % TAU;
        let normalized_phase = phase / TAU; // Phase normalized to [0, 1]
        if normalized_phase < 0.5 {
            4.0 * normalized_phase - 1.0
        } else {
            3.0 - 4.0 * normalized_phase
        }
    }
}

impl Compute for Triangle {
    fn compute(&self, context: ComputeContext) -> f32 {
        self.compute_with_hz(context, self.hz())
    }
}

#[derive(Clone)]
pub struct Sawtooth {
    pub hz: Func,
}

impl PeriodicCompute for Sawtooth {
    fn hz(&self) -> Func {
        self.hz.clone()
    }
    fn compute_with_hz(&self, context: ComputeContext, hz: Func) -> f32 {
        let phase = (TAU * context.time_elapsed.as_secs_f32() * hz.compute(context)) % TAU;
        (2.0 * (phase / TAU)) - 1.0
    }
}

impl Compute for Sawtooth {
    fn compute(&self, context: ComputeContext) -> f32 {
        self.compute_with_hz(context, self.hz())
    }
}
