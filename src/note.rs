use std::{
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};

use rayon::prelude::*;

use crate::prelude::*;

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

        let stereo = AtomicBool::new(false);

        let samples = (0..sample_count)
            .into_par_iter()
            .map(|sample| {
                let progress = sample as f32 / sample_count as f32;

                let time_elapsed = Duration::from_secs_f32(sample as f32 / sample_rate);

                let context = ComputeContext {
                    progress,
                    time_elapsed,
                    sample,
                };

                let val = self.instrument.compute(context);
                if val == 0. {
                    return RenderedSample::default();
                }

                let pan = self.pan.compute(context).clamp(0., 1.);
                if pan != 0.5 && !stereo.load(Ordering::Relaxed) {
                    stereo.store(true, Ordering::Relaxed);
                }

                RenderedSample::from_pan(val, pan)
            })
            .collect();

        RenderOutput {
            samples,
            stereo: stereo.load(Ordering::Relaxed),
            sample_rate,
        }
    }
}
