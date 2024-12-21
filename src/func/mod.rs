use std::{sync::Arc, time::Duration};

pub mod easing;
pub mod math;
pub mod modifier;
#[cfg(feature = "noise")]
pub mod noise;
pub mod periodic;

pub trait Compute: Send + Sync {
    fn compute(&self, context: ComputeContext) -> f32;
}

impl<T: Fn(ComputeContext) -> f32 + Send + Sync> Compute for T {
    fn compute(&self, context: ComputeContext) -> f32 {
        self(context)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ComputeContext {
    pub progress: f32,
    pub time_elapsed: Duration,
    pub sample: usize,
}

#[derive(Clone)]
pub struct Func(pub Arc<dyn Compute>);

impl Compute for Func {
    fn compute(&self, context: ComputeContext) -> f32 {
        self.0.compute(context)
    }
}

pub trait IntoFunc {
    fn f(self) -> Func;
}

impl<T: Compute + 'static> IntoFunc for T {
    fn f(self) -> Func {
        Func(Arc::new(self))
    }
}
