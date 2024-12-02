use std::ops::{Add, Div, Mul, Sub};

use crate::prelude::*;

impl Add for Func {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        (move |context| self.compute(context) + rhs.compute(context)).f()
    }
}

impl Sub for Func {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        (move |context| self.compute(context) - rhs.compute(context)).f()
    }
}

impl Mul for Func {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        (move |context| self.compute(context) * rhs.compute(context)).f()
    }
}

impl Div for Func {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        (move |context| self.compute(context) / rhs.compute(context)).f()
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::prelude::*;

    #[test]
    fn add_funcs() {
        let f = (0., 1.).f() + 1.0.f();
        assert_eq!(f.compute(context(0.)), 1.);
        assert_eq!(f.compute(context(0.5)), 1.5);
        assert_eq!(f.compute(context(1.)), 2.);
    }

    #[test]
    fn sub_funcs() {
        let f = (0., 1.).f() - 1.0.f();
        assert_eq!(f.compute(context(0.)), -1.);
        assert_eq!(f.compute(context(0.5)), -0.5);
        assert_eq!(f.compute(context(1.)), 0.);
    }

    #[test]
    fn mul_funcs() {
        let f = (0., 1.).f() * 2.0.f();
        assert_eq!(f.compute(context(0.)), 0.);
        assert_eq!(f.compute(context(0.5)), 1.);
        assert_eq!(f.compute(context(1.)), 2.);
    }

    #[test]
    fn div_funcs() {
        let f = (0., 1.).f() / 2.0.f();
        assert_eq!(f.compute(context(0.)), 0.);
        assert_eq!(f.compute(context(0.5)), 0.25);
        assert_eq!(f.compute(context(1.)), 0.5);
    }

    fn context(progress: f32) -> ComputeContext {
        ComputeContext {
            progress,
            time_elapsed: Duration::from_secs_f32(progress),
            sample: (progress * 44100.) as usize,
        }
    }
}
