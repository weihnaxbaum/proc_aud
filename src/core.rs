use std::{rc::Rc, time::Duration};

#[cfg(feature = "wav")]
use hound::{SampleFormat, WavSpec, WavWriter};

#[derive(Clone, Default)]
pub struct Track<'a> {
    pub notes: Vec<Note<'a>>,
}

impl<'a> Track<'a> {
    pub fn render(&self, sample_rate: f32) -> RenderOutput {
        let mut output = RenderOutput {
            samples: vec![],
            stereo: false,
        };
        for note in &self.notes {
            let start_sample = (note.start.as_secs_f32() * sample_rate) as usize;
            let end_sample = ((note.start + note.duration).as_secs_f32() * sample_rate) as usize;

            for current_sample in start_sample..end_sample {
                let note_progress =
                    (current_sample - start_sample) as f32 / (end_sample - start_sample) as f32;

                let amplitude = (note.amplitude)(note_progress);
                if amplitude <= 0. {
                    continue;
                }

                let hz = (note.hz)(note_progress);
                if hz <= 0. {
                    continue;
                }

                let note_time_elapsed =
                    Duration::from_secs_f32((current_sample - start_sample) as f32 / sample_rate);

                let sample_data = SampleData {
                    note_time_elapsed,
                    hz,
                };

                let val = note.instrument.sample(sample_data);
                let pan = (note.pan)(note_progress).clamp(0., 1.);
                if pan != 0.5 {
                    output.stereo = true;
                }
                let mut rendered_sample = RenderedSample::from_pan(val, pan);

                rendered_sample.left *= amplitude;
                rendered_sample.right *= amplitude;

                if output.samples.len() <= current_sample {
                    output
                        .samples
                        .resize_with(current_sample + 1, RenderedSample::default);
                }

                output.samples[current_sample].left += rendered_sample.left;
                output.samples[current_sample].right += rendered_sample.right;
            }
        }
        output
    }
    #[cfg(feature = "wav")]
    pub fn save_wav(
        &self,
        path: &str,
        sample_rate: f32,
        bits_per_sample: u16,
    ) -> hound::Result<()> {
        let rendered = self.render(sample_rate);
        let channels = if rendered.stereo { 2 } else { 1 };
        let spec = WavSpec {
            channels,
            sample_rate: sample_rate as u32,
            bits_per_sample,
            sample_format: SampleFormat::Float,
        };
        let mut writer = WavWriter::create(path, spec)?;
        for sample in rendered.samples {
            writer.write_sample(sample.left)?;
            if rendered.stereo {
                writer.write_sample(sample.right)?;
            }
        }
        writer.finalize()?;
        Ok(())
    }
}

#[derive(Clone)]
pub struct RenderOutput {
    pub samples: Vec<RenderedSample>,
    pub stereo: bool,
}

#[derive(Clone)]
pub struct Note<'a> {
    pub start: Duration,
    pub duration: Duration,
    pub instrument: &'a dyn Instrument,
    pub hz: Func,
    pub amplitude: Func,
    pub pan: Func,
}

pub trait Instrument {
    fn sample(&self, data: SampleData) -> f32;
}

#[derive(Clone, Copy, Debug)]
pub struct SampleData {
    pub note_time_elapsed: Duration,
    pub hz: f32,
}

pub type Func = Rc<dyn Fn(f32) -> f32>;

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
