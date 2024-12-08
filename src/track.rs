use std::time::Duration;

use crate::{note::TimedNote, prelude::*};

#[derive(Clone, Default)]
pub struct Track {
    pub notes: Vec<TimedNote>,
}

impl Track {
    pub fn from_cycle(
        rhythm: &[Duration],
        instruments: &[Func],
        pans: &[Func],
        len: usize,
    ) -> Self {
        let mut notes = Vec::with_capacity(len);
        let mut start = Duration::ZERO;
        for i in 0..len {
            let duration = rhythm[i % rhythm.len()];
            notes.push(
                Note {
                    duration,
                    instrument: instruments[i % instruments.len()].clone(),
                    pan: pans[i % pans.len()].clone(),
                }
                .start_at(start),
            );
            start += duration;
        }
        Track { notes }
    }

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
