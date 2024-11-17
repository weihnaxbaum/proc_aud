use std::f32::consts::{PI, TAU};

use crate::prelude::*;

pub struct Sine;

impl Instrument for Sine {
    fn sample(&self, data: SampleData) -> f32 {
        (TAU * data.note_time_elapsed.as_secs_f32() * data.hz).sin()
    }
}

pub struct Square;

impl Instrument for Square {
    fn sample(&self, data: SampleData) -> f32 {
        let phase = (TAU * data.note_time_elapsed.as_secs_f32() * data.hz) % TAU;
        if phase < PI {
            1.0
        } else {
            -1.0
        }
    }
}

pub struct Triangle;

impl Instrument for Triangle {
    fn sample(&self, data: SampleData) -> f32 {
        let phase = (TAU * data.note_time_elapsed.as_secs_f32() * data.hz) % TAU;
        let normalized_phase = phase / TAU; // Phase normalized to [0, 1]
        if normalized_phase < 0.5 {
            4.0 * normalized_phase - 1.0
        } else {
            3.0 - 4.0 * normalized_phase
        }
    }
}

pub struct Sawtooth;

impl Instrument for Sawtooth {
    fn sample(&self, data: SampleData) -> f32 {
        let phase = (TAU * data.note_time_elapsed.as_secs_f32() * data.hz) % TAU;
        (2.0 * (phase / TAU)) - 1.0
    }
}

pub struct FrequencyAdjustedInstrument<'a> {
    pub instrument: &'a dyn Instrument,
    pub multiplier: f32,
}

impl<'a> Instrument for FrequencyAdjustedInstrument<'a> {
    fn sample(&self, mut data: SampleData) -> f32 {
        data.hz *= self.multiplier;
        self.instrument.sample(data)
    }
}

pub struct CombinedInstruments<'a> {
    pub instruments: Vec<&'a dyn Instrument>,
}

impl<'a> Instrument for CombinedInstruments<'a> {
    fn sample(&self, data: SampleData) -> f32 {
        self.instruments.iter().map(|i| i.sample(data)).sum()
    }
}

pub struct TimbreInstrument<'a> {
    pub timbre: Vec<(f32, f32)>,
    pub instrument: &'a dyn Instrument,
}

impl<'a> Instrument for TimbreInstrument<'a> {
    fn sample(&self, data: SampleData) -> f32 {
        self.timbre
            .iter()
            .map(|(multiplier, amplitude)| {
                let instrument = FrequencyAdjustedInstrument {
                    instrument: self.instrument,
                    multiplier: *multiplier,
                };
                instrument.sample(data) * amplitude
            })
            .sum()
    }
}
