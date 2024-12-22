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
        let mut track = Track::default();
        track.cycle(Duration::ZERO, rhythm, instruments, pans, len);
        track
    }

    pub fn cycle(
        &mut self,
        mut start: Duration,
        rhythm: &[Duration],
        instruments: &[Func],
        pans: &[Func],
        len: usize,
    ) -> &mut Self {
        self.notes.reserve(len);
        for i in 0..len {
            let duration = rhythm[i % rhythm.len()];
            self.notes.push(
                Note {
                    duration,
                    instrument: instruments[i % instruments.len()].clone(),
                    pan: pans[i % pans.len()].clone(),
                }
                .start_at(start),
            );
            start += duration;
        }
        self
    }

    pub fn from_chord(durations: &[Duration], instruments: &[Func], pans: &[Func]) -> Self {
        let mut track = Track::default();
        track.chord(Duration::ZERO, durations, instruments, pans);
        track
    }

    pub fn chord(
        &mut self,
        start: Duration,
        durations: &[Duration],
        instruments: &[Func],
        pans: &[Func],
    ) -> &mut Self {
        let len = instruments.len();
        self.notes.reserve(len);
        for i in 0..len {
            self.notes.push(
                Note {
                    duration: durations[i % durations.len()],
                    instrument: instruments[i].clone(),
                    pan: pans[i % pans.len()].clone(),
                }
                .start_at(start),
            );
        }
        self
    }

    /// Repeats the Track `n` times.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::time::Duration;
    ///
    /// use proc_aud::prelude::*;
    ///
    /// let mut track = Track::default();
    /// track.notes.push(
    ///     Note {
    ///         duration: Duration::from_secs(1),
    ///         instrument: Sine { hz: 440.0.f() }.f() * Exp(0.5).f(),
    ///         pan: 0.5.f(),
    ///     }
    ///     .into(),
    /// );
    /// assert_eq!(track.duration(), track.repeat(0).duration());
    /// assert_eq!(track.repeat(2).duration(), Duration::from_secs(3));
    /// ```
    pub fn repeat(&mut self, n: usize) -> &mut Self {
        let duration = self.duration();
        let mut notes = self.notes.clone();
        self.notes.reserve(n * self.notes.len());
        for _ in 1..=n {
            self.notes.extend(notes.iter_mut().map(|note| {
                note.start += duration;
                note.clone()
            }));
        }
        self
    }

    /// Calculates the duration of the Track.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::time::Duration;
    ///
    /// use proc_aud::prelude::*;
    ///
    /// let mut track = Track::default();
    ///
    /// let instrument = Sine { hz: 440.0.f() }.f();
    /// let pan = 0.5.f();
    ///
    /// track.notes.push( // 2 secs
    ///     Note {
    ///         duration: Duration::from_secs(2),
    ///         instrument: instrument.clone(),
    ///         pan: pan.clone(),
    ///     }
    ///     .start_at(Duration::ZERO),
    /// );
    /// track.notes.push( // 2 secs
    ///     Note {
    ///         duration: Duration::from_secs_f32(0.5),
    ///         instrument: instrument.clone(),
    ///         pan: pan.clone(),
    ///     }
    ///     .start_at(Duration::from_secs_f32(1.5)),
    /// );
    /// track.notes.push( // 2.5 secs
    ///     Note {
    ///         duration: Duration::from_secs_f32(1.5),
    ///         instrument,
    ///         pan,
    ///     }
    ///     .start_at(Duration::from_secs(1)),
    /// );
    ///
    /// assert_eq!(track.duration(), Duration::from_secs_f32(2.5));
    /// ```
    pub fn duration(&self) -> Duration {
        match self
            .notes
            .iter()
            .map(|timed_note| timed_note.start + timed_note.note.duration)
            .max()
        {
            Some(v) => v,
            None => Duration::ZERO,
        }
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
