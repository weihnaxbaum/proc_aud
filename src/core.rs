use std::{rc::Rc, time::Duration};

#[cfg(feature = "wav")]
use hound::{SampleFormat, WavSpec, WavWriter};

#[derive(Clone, Default)]
pub struct Track {
    pub notes: Vec<Note>,
}

impl Track {
    pub fn render(&self, sample_rate: f32) -> RenderOutput {
        let mut output = RenderOutput {
            samples: vec![],
            stereo: false,
            sample_rate,
        };
        for note in &self.notes {
            let rendered_note = note.render(sample_rate);

            if rendered_note.stereo {
                output.stereo = true;
            }

            let start_sample = (note.start.as_secs_f32() * sample_rate) as usize;
            let end_sample = ((note.start + note.duration).as_secs_f32() * sample_rate) as usize;

            if output.samples.len() <= end_sample {
                output
                    .samples
                    .resize_with(end_sample + 1, RenderedSample::default);
            }

            for current_sample in start_sample..end_sample {
                output.samples[current_sample].left += rendered_note.samples[current_sample].left;
                output.samples[current_sample].right += rendered_note.samples[current_sample].right;
            }
        }
        output
    }
}

#[derive(Clone)]
pub struct RenderOutput {
    pub samples: Vec<RenderedSample>,
    pub stereo: bool,
    pub sample_rate: f32,
}

impl RenderOutput {
    #[cfg(feature = "wav")]
    pub fn save_wav(&self, path: &str, bits_per_sample: u16) -> hound::Result<()> {
        let channels = if self.stereo { 2 } else { 1 };
        let spec = WavSpec {
            channels,
            sample_rate: self.sample_rate as u32,
            bits_per_sample,
            sample_format: SampleFormat::Float,
        };
        let mut writer = WavWriter::create(path, spec)?;
        for sample in &self.samples {
            writer.write_sample(sample.left)?;
            if self.stereo {
                writer.write_sample(sample.right)?;
            }
        }
        writer.finalize()?;
        Ok(())
    }
}

#[derive(Clone)]
pub struct Note {
    pub start: Duration,
    pub duration: Duration,
    pub instrument: Func,
    pub amp: Func,
    pub pan: Func,
}

impl Note {
    pub fn render(&self, sample_rate: f32) -> RenderOutput {
        let start_sample = (self.start.as_secs_f32() * sample_rate) as usize;
        let end_sample = ((self.start + self.duration).as_secs_f32() * sample_rate) as usize;

        let mut output = RenderOutput {
            samples: vec![RenderedSample::default(); end_sample],
            stereo: false,
            sample_rate,
        };

        for sample in start_sample..end_sample {
            let progress = (sample - start_sample) as f32 / (end_sample - start_sample) as f32;

            let time_elapsed =
                Duration::from_secs_f32((sample - start_sample) as f32 / sample_rate);

            let context = ComputeContext {
                progress,
                time_elapsed,
                sample,
            };

            let val = self.instrument.compute(context);

            let amp = self.amp.compute(context);
            if amp <= 0. {
                continue;
            }

            let pan = self.pan.compute(context).clamp(0., 1.);
            if pan != 0.5 {
                output.stereo = true;
            }
            let mut rendered_sample = RenderedSample::from_pan(val, pan);

            rendered_sample.left *= amp;
            rendered_sample.right *= amp;

            if output.samples.len() <= sample {
                output
                    .samples
                    .resize_with(sample + 1, RenderedSample::default);
            }

            output.samples[sample].left += rendered_sample.left;
            output.samples[sample].right += rendered_sample.right;
        }
        output
    }
}

pub trait Compute {
    fn compute(&self, context: ComputeContext) -> f32;
}

impl<T: Fn(ComputeContext) -> f32> Compute for T {
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

pub type Func = Rc<dyn Compute>;

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
