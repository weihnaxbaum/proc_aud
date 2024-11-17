use std::time::Duration;

#[derive(Clone, Default)]
pub struct Track<'a> {
    pub notes: Vec<Note<'a>>,
}

impl<'a> Track<'a> {
    pub fn render(&self, sample_rate: f32) -> Vec<RenderedSample> {
        let mut result = vec![];
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
                let mut rendered_sample = RenderedSample::from_pan(val, pan);

                rendered_sample.left *= amplitude;
                rendered_sample.right *= amplitude;

                if result.len() <= current_sample {
                    result.resize_with(current_sample + 1, RenderedSample::default);
                }

                result[current_sample].left += rendered_sample.left;
                result[current_sample].right += rendered_sample.right;
            }
        }
        result
    }
}

#[derive(Clone)]
pub struct Note<'a> {
    pub start: Duration,
    pub duration: Duration,
    pub instrument: &'a dyn Instrument,
    pub hz: Func<'a>,
    pub amplitude: Func<'a>,
    pub pan: Func<'a>,
}

pub trait Instrument {
    fn sample(&self, data: SampleData) -> f32;
}

#[derive(Clone, Copy, Debug)]
pub struct SampleData {
    note_time_elapsed: Duration,
    hz: f32,
}

impl SampleData {
    pub fn note_time_elapsed(&self) -> Duration {
        self.note_time_elapsed
    }
    pub fn hz(&self) -> f32 {
        self.hz
    }
}

pub type Func<'a> = &'a dyn Fn(f32) -> f32;

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

#[cfg(test)]
mod tests {
    use std::f32::consts::TAU;

    use super::*;

    struct SineWave;

    impl Instrument for SineWave {
        fn sample(&self, data: SampleData) -> f32 {
            (TAU * data.note_time_elapsed().as_secs_f32() * data.hz()).sin()
        }
    }

    #[test]
    fn simple() {
        let mut track = Track::default();
        track.notes.push(Note {
            start: Duration::from_secs_f32(0.5),
            duration: Duration::from_secs_f32(2.5),
            instrument: &SineWave,
            hz: &|_| 0.5,
            amplitude: &|_| 0.5,
            pan: &|_| 0.5,
        });
        let rendered = track.render(2.);
        assert_approx_eq(rendered[0], RenderedSample::ZERO);
        assert_approx_eq(rendered[1], RenderedSample::ZERO);
        assert_approx_eq(rendered[2], RenderedSample::center(0.25));
        assert_approx_eq(rendered[3], RenderedSample::ZERO);
        assert_approx_eq(rendered[4], RenderedSample::center(-0.25));
    }

    #[test]
    fn multiple() {
        let mut track = Track::default();
        track.notes.push(Note {
            start: Duration::ZERO,
            duration: Duration::from_secs(1),
            instrument: &SineWave,
            hz: &|_| 1.,
            amplitude: &|_| 0.5,
            pan: &|_| 0.5,
        });
        track.notes.push(Note {
            start: Duration::ZERO,
            duration: Duration::from_secs(1),
            instrument: &SineWave,
            hz: &|_| 1.25,
            amplitude: &|_| 1.,
            pan: &|_| 0.5,
        });
        let rendered = track.render(4.);
        assert_approx_eq(rendered[0], RenderedSample::center(0. + 0.));
        assert_approx_eq(rendered[1], RenderedSample::center(0.25 + 0.46193975));
        assert_approx_eq(rendered[2], RenderedSample::center(0. + -0.35355338));
        assert_approx_eq(rendered[3], RenderedSample::center(-0.25 + -0.19134171));
    }

    #[test]
    fn pan() {
        let mut track = Track::default();
        track.notes.push(Note {
            start: Duration::ZERO,
            duration: Duration::from_secs(1),
            instrument: &SineWave,
            hz: &|_| 1.,
            amplitude: &|_| 1.,
            pan: &|_| 0.25,
        });
        let rendered = track.render(4.);
        assert_approx_eq(rendered[0], RenderedSample::ZERO);
        assert_approx_eq(rendered[1], RenderedSample::new(0.75, 0.25));
        assert_approx_eq(rendered[2], RenderedSample::new(0., 0.));
        assert_approx_eq(rendered[3], RenderedSample::new(-0.75, -0.25));
    }

    #[test]
    fn pitch_shift() {
        let mut track = Track::default();
        track.notes.push(Note {
            start: Duration::ZERO,
            duration: Duration::from_secs(1),
            instrument: &SineWave,
            hz: &|x| x,
            amplitude: &|_| 1.,
            pan: &|_| 0.5,
        });
        let rendered = track.render(5.);
        assert_approx_eq(rendered[0], RenderedSample::ZERO);
        assert_approx_eq(rendered[1], RenderedSample::center(0.124344945));
        assert_approx_eq(rendered[2], RenderedSample::center(0.42216396));
        assert_approx_eq(rendered[3], RenderedSample::center(0.38525662));
        assert_approx_eq(rendered[4], RenderedSample::center(-0.38525662));
    }

    fn assert_approx_eq(left: RenderedSample, right: RenderedSample) {
        let epsilon = 0.0000005;
        assert!(
            (left.left - right.left).abs() < epsilon,
            "left.left: {}, right.left: {}",
            left.left,
            right.left,
        );
        assert!(
            (left.right - right.right).abs() < epsilon,
            "left.right: {}, right.right: {}",
            left.right,
            right.right,
        );
    }
}
