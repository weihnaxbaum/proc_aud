use std::{ops::Mul, sync::Arc, time::Duration};

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

impl Mul<f32> for ComputeContext {
    type Output = Self;
    fn mul(mut self, rhs: f32) -> Self::Output {
        self.progress *= rhs;
        self.time_elapsed = Duration::from_secs_f32(self.time_elapsed.as_secs_f32() * rhs);
        self.sample = (self.sample as f32 * rhs) as usize;
        self
    }
}

#[derive(Clone)]
pub struct Func(pub Arc<dyn Compute>);

impl IntoFunc for Func {
    fn f(self) -> Func {
        self
    }
}

// Func doesn't impl `Compute` so that it can impl `IntoFunc`
// without confilicting trait impls
impl Func {
    pub fn compute(&self, context: ComputeContext) -> f32 {
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
