use std::time::Duration;

use fastrand::Rng;

use crate::prelude::*;

/// Can generate a "random" track from a seed.
#[derive(Clone)]
pub struct Gen {
    pub instrument: Func,
    /// Must not be zero.
    pub note_duration: Duration,
    pub min_semitone_shift: f32,
    pub max_semitone_shift: f32,
    pub max_semitone_jump: u16,
    pub seed: u64,
}

impl Default for Gen {
    fn default() -> Self {
        Self {
            instrument: 0.0.f(),
            note_duration: Duration::from_secs_f32(0.5),
            min_semitone_shift: -12.0,
            max_semitone_shift: 12.0,
            max_semitone_jump: 6,
            seed: 0,
        }
    }
}

impl Gen {
    /// Generates the track.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::time::Duration;
    ///
    /// use proc_aud::prelude::*;
    ///
    /// let gen = Gen {
    ///     note_duration: Duration::from_secs_f32(0.25),
    ///     ..Default::default()
    /// };
    /// let track = gen.gen(Duration::from_secs_f32(1.5));
    /// assert_eq!(track.notes.len(), 6);
    /// ```
    pub fn gen(&self, duration: Duration) -> Track {
        let segments = (duration.as_secs_f32() / self.note_duration.as_secs_f32()) as u32;
        let mut out = Track::default();
        let mut rng = Rng::with_seed(self.seed);
        let mut semitone_target = (self.min_semitone_shift
            + (self.max_semitone_shift - self.min_semitone_shift) * rng.f32())
            as i32;
        let mut semitone_shift = 0;
        for i in 0..segments {
            let semitone_diff = semitone_target - semitone_shift;
            semitone_shift += if semitone_diff > self.max_semitone_jump as i32 {
                self.max_semitone_jump as i32
            } else if semitone_diff < -(self.max_semitone_jump as i32) {
                -(self.max_semitone_jump as i32)
            } else {
                semitone_diff
            };
            if semitone_shift == semitone_target {
                semitone_target = (self.min_semitone_shift
                    + (self.max_semitone_shift - self.min_semitone_shift) * rng.f32())
                    as i32;
            }
            let freq_mul = 2.0f32.powf(1. / 12.).powi(semitone_shift);
            let instrument = self.instrument.clone();
            let instrument = move |context| instrument.compute(context * freq_mul);
            out.notes.push(
                Note::center(self.note_duration, instrument).start_at(i * self.note_duration),
            );
        }
        out
    }
}
