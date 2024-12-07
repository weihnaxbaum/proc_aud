#[cfg(feature = "wav")]
use std::io::{Seek, Write};
use std::{rc::Rc, time::Duration};

#[cfg(feature = "wav")]
use hound::{SampleFormat, WavSpec, WavWriter};

#[derive(Clone, Default)]
pub struct Track {
    pub notes: Vec<TimedNote>,
}

impl Track {
    pub fn render(&self, sample_rate: f32) -> RenderOutput {
        let mut output = RenderOutput {
            samples: vec![],
            stereo: false,
            sample_rate,
        };
        for timed_note in &self.notes {
            let rendered_note = timed_note.note.render(sample_rate);

            if rendered_note.stereo {
                output.stereo = true;
            }

            let start_sample = (timed_note.start.as_secs_f32() * sample_rate) as usize;
            let end_sample = ((timed_note.start + timed_note.note.duration).as_secs_f32()
                * sample_rate) as usize;

            if output.samples.len() <= end_sample {
                output
                    .samples
                    .resize_with(end_sample + 1, RenderedSample::default);
            }

            for (i, sample) in rendered_note.samples.into_iter().enumerate() {
                output.samples[i + start_sample].left += sample.left;
                output.samples[i + start_sample].right += sample.right;
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

    #[cfg(feature = "wav")]
    pub fn save_wav(&self, path: &str, encoding: WavEncoding) -> hound::Result<()> {
        let channels = if self.stereo { 2 } else { 1 };
        let spec = WavSpec {
            channels,
            sample_rate: self.sample_rate as u32,
            bits_per_sample: match encoding {
                WavEncoding::I8 => 8,
                WavEncoding::I16 => 16,
                WavEncoding::I24 => 24,
                WavEncoding::I32 => 32,
                WavEncoding::F32 => 32,
            },
            sample_format: match encoding {
                WavEncoding::I8 | WavEncoding::I16 | WavEncoding::I24 | WavEncoding::I32 => {
                    SampleFormat::Int
                }
                WavEncoding::F32 => SampleFormat::Float,
            },
        };
        let mut writer = WavWriter::create(path, spec)?;
        for sample in &self.samples {
            wav_write_mono_sample(&mut writer, sample.left, encoding)?;
            if self.stereo {
                wav_write_mono_sample(&mut writer, sample.right, encoding)?;
            }
        }
        writer.finalize()?;
        Ok(())
    }
}

#[cfg(feature = "wav")]
fn wav_write_mono_sample<W: Write + Seek>(
    writer: &mut WavWriter<W>,
    sample: f32,
    encoding: WavEncoding,
) -> hound::Result<()> {
    match encoding {
        WavEncoding::I8 => writer.write_sample((sample * i8::MAX as f32) as i8),
        WavEncoding::I16 => writer.write_sample((sample * i16::MAX as f32) as i16),
        WavEncoding::I24 => writer.write_sample((sample * 0x7FFFFF as f32) as i32),
        WavEncoding::I32 => writer.write_sample((sample * i32::MAX as f32) as i32),
        WavEncoding::F32 => writer.write_sample(sample),
    }?;
    Ok(())
}

#[cfg(feature = "wav")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WavEncoding {
    I8,
    I16,
    I24,
    I32,
    F32,
}

#[derive(Clone)]
pub struct TimedNote {
    pub start: Duration,
    pub note: Note,
}

impl From<Note> for TimedNote {
    fn from(note: Note) -> Self {
        note.start_at(Duration::ZERO)
    }
}

#[derive(Clone)]
pub struct Note {
    pub duration: Duration,
    pub instrument: Func,
    pub pan: Func,
}

impl Note {
    pub fn start_at(self, start: Duration) -> TimedNote {
        TimedNote { start, note: self }
    }

    pub fn render(&self, sample_rate: f32) -> RenderOutput {
        let sample_count = (self.duration.as_secs_f32() * sample_rate) as usize;

        let mut samples = Vec::with_capacity(sample_count);
        let mut stereo = false;

        for sample in 0..sample_count {
            let progress = sample as f32 / sample_count as f32;

            let time_elapsed = Duration::from_secs_f32(sample as f32 / sample_rate);

            let context = ComputeContext {
                progress,
                time_elapsed,
                sample,
            };

            let val = self.instrument.compute(context);
            if val == 0. {
                samples.push(RenderedSample::default());
                continue;
            }

            let pan = self.pan.compute(context).clamp(0., 1.);
            if pan != 0.5 {
                stereo = true;
            }

            let rendered_sample = RenderedSample::from_pan(val, pan);
            samples.push(rendered_sample);
        }
        RenderOutput {
            samples,
            stereo,
            sample_rate,
        }
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

pub trait IntoFunc {
    fn f(self) -> Func;
}

impl<T: Compute + 'static> IntoFunc for T {
    fn f(self) -> Func {
        Func(Rc::new(self))
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ComputeContext {
    pub progress: f32,
    pub time_elapsed: Duration,
    pub sample: usize,
}

#[derive(Clone)]
pub struct Func(pub Rc<dyn Compute>);

impl Compute for Func {
    fn compute(&self, context: ComputeContext) -> f32 {
        self.0.compute(context)
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
