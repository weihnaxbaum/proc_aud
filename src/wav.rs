use std::io::{Read, Seek, Write};

use hound::{SampleFormat, WavReader, WavSpec, WavWriter};

use crate::prelude::*;

impl RenderOutput {
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

    pub fn from_wav<R: Read>(reader: R) -> hound::Result<Self> {
        let mut wav_reader = WavReader::new(reader)?;
        let spec = wav_reader.spec();
        let stereo = match spec.channels {
            1 => false,
            2 => true,
            _ => return Err(hound::Error::Unsupported),
        };
        let sample_rate = spec.sample_rate as f32;
        let samples = if spec.sample_format == SampleFormat::Float {
            let hound_samples = wav_reader.samples::<f32>();
            let mut samples: Vec<RenderedSample> = Vec::with_capacity(hound_samples.len());
            for (i, sample) in hound_samples.enumerate() {
                if stereo && i % 2 == 1 {
                    let last = samples.last_mut().unwrap();
                    last.right = sample?;
                } else {
                    samples.push(RenderedSample::center(sample?));
                }
            }
            samples
        } else {
            let normalization = match spec.bits_per_sample {
                8 => u8::MAX as f32,
                16 => u16::MAX as f32,
                24 => 0xFFFFFF as f32,
                32 => u32::MAX as f32,
                _ => return Err(hound::Error::Unsupported),
            };
            let hound_samples = wav_reader.samples::<i32>();
            let mut samples: Vec<RenderedSample> = Vec::with_capacity(hound_samples.len());
            for (i, sample) in hound_samples.enumerate() {
                if stereo && i % 2 == 1 {
                    let last = samples.last_mut().unwrap();
                    last.right = sample? as f32 / i32::MAX as f32;
                } else {
                    samples.push(RenderedSample::center(sample? as f32 / normalization));
                }
            }
            samples
        };

        let render_output = Self {
            samples,
            sample_rate,
            stereo,
        };
        Ok(render_output)
    }
}

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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WavEncoding {
    I8,
    I16,
    I24,
    I32,
    F32,
}
