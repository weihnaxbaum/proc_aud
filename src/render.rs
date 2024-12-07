#[derive(Clone)]
pub struct RenderOutput {
    pub samples: Vec<RenderedSample>,
    pub stereo: bool,
    pub sample_rate: f32,
}

impl RenderOutput {
    pub fn normalize(&mut self) -> &mut Self {
        let Some(first) = self.samples.first() else {
            return self;
        };
        let mut abs_max = first.left;
        for sample in self.samples.iter() {
            if abs_max < sample.left.abs() {
                abs_max = sample.left.abs();
            }
            if abs_max < sample.right.abs() {
                abs_max = sample.right.abs();
            }
        }
        if abs_max == 0. {
            return self;
        }
        for sample in self.samples.iter_mut() {
            sample.left /= abs_max;
            sample.right /= abs_max;
        }
        self
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RenderedSample {
    pub left: f32,
    pub right: f32,
}

impl RenderedSample {
    pub const ZERO: Self = RenderedSample {
        left: 0.,
        right: 0.,
    };
    pub fn new(left: f32, right: f32) -> Self {
        Self { left, right }
    }
    pub fn from_pan(val: f32, pan: f32) -> Self {
        Self {
            left: val * (1. - pan),
            right: val * pan,
        }
    }
    pub fn center(val: f32) -> Self {
        Self::new(val, val)
    }
}
