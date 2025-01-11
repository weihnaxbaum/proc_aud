use std::ops::{Add, AddAssign};

#[derive(Clone)]
pub struct RenderOutput {
    pub samples: Vec<RenderedSample>,
    pub stereo: bool,
    pub sample_rate: f32,
}

impl Add for RenderOutput {
    type Output = Self;

    fn add(mut self, mut rhs: Self) -> Self::Output {
        if self.sample_rate > rhs.sample_rate {
            rhs.resample(self.sample_rate);
        } else if self.sample_rate < rhs.sample_rate {
            self.resample(rhs.sample_rate);
        }
        for (i, sample) in rhs.samples.into_iter().enumerate() {
            self.samples[i] += sample;
        }
        self.stereo |= rhs.stereo;
        self
    }
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

    /// Changes the sample rate without affecting the speed/duration of the sound.
    ///
    /// A lower sample rate results in reduced quality but also less samples to store.
    ///
    /// A higher sample rate results in equal quality while also storing more samples.
    /// This is mainly used to synchonize the samples with another sound
    /// with a higher sample rate to add both sounds together.
    ///
    /// # Examples
    ///
    /// ```
    /// use proc_aud::prelude::*;
    ///
    /// // 1 sec, 2 samples
    /// let mut render_output = RenderOutput {
    ///     samples: vec![RenderedSample::center(-1.), RenderedSample::center(1.)],
    ///     sample_rate: 2.,
    ///     stereo: false,
    /// };
    ///
    /// // 1 sec, 4 samples (lossless)
    /// render_output.resample(4.);
    /// assert_eq!(render_output.samples.len(), 4);
    /// assert_eq!(render_output.samples[0], RenderedSample::center(-1.));
    /// assert_eq!(render_output.samples[1], RenderedSample::center(-1.));
    /// assert_eq!(render_output.samples[2], RenderedSample::center(1.));
    /// assert_eq!(render_output.samples[3], RenderedSample::center(1.));
    ///
    /// // Reset sample rate to 2: 1 sec, 2 samples
    /// render_output.resample(2.);
    /// assert_eq!(render_output.samples.len(), 2);
    /// assert_eq!(render_output.samples[0], RenderedSample::center(-1.));
    /// assert_eq!(render_output.samples[1], RenderedSample::center(1.));
    ///
    /// // 1 sec, 1 sample (lossy)
    /// render_output.resample(1.);
    /// assert_eq!(render_output.samples.len(), 1);
    /// assert_eq!(render_output.samples[0], RenderedSample::center(-1.));
    ///
    /// // Due to the information loss, the initial values can't be recreated.
    /// // 1 sec, 2 samples
    /// render_output.resample(2.);
    /// assert_eq!(render_output.samples.len(), 2);
    /// assert_eq!(render_output.samples[0], RenderedSample::center(-1.));
    /// assert_eq!(render_output.samples[0], RenderedSample::center(-1.));
    /// ```
    pub fn resample(&mut self, sample_rate: f32) -> &mut Self {
        let sample_rate_ratio = sample_rate / self.sample_rate;
        let len = (self.samples.len() as f32 * sample_rate_ratio) as usize;
        let mut new = Vec::with_capacity(len);
        for i in 0..len {
            let old_i = (i as f32 / sample_rate_ratio) as usize;
            let old_sample = self.samples[old_i];
            new.push(old_sample);
        }
        self.samples = new;
        self.sample_rate = sample_rate;
        self
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RenderedSample {
    pub left: f32,
    pub right: f32,
}

impl AddAssign for RenderedSample {
    fn add_assign(&mut self, rhs: Self) {
        self.left += rhs.left;
        self.right += rhs.right;
    }
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
