use std::io::{Seek, Write};

use hound::{SampleFormat, WavSpec, WavWriter};

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
