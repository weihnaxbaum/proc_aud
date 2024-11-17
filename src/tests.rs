use std::time::Duration;

use crate::prelude::*;

#[test]
fn simple() {
    let mut track = Track::default();
    track.notes.push(Note {
        start: Duration::from_secs_f32(0.5),
        duration: Duration::from_secs_f32(2.5),
        instrument: &Sine,
        hz: &|_| 0.5,
        amplitude: &|_| 0.5,
        pan: &|_| 0.5,
    });
    let rendered = track.render(2.).samples;
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
        instrument: &Sine,
        hz: &|_| 1.,
        amplitude: &|_| 0.5,
        pan: &|_| 0.5,
    });
    track.notes.push(Note {
        start: Duration::ZERO,
        duration: Duration::from_secs(1),
        instrument: &Sine,
        hz: &|_| 1.25,
        amplitude: &|_| 1.,
        pan: &|_| 0.5,
    });
    let rendered = track.render(4.).samples;
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
        instrument: &Sine,
        hz: &|_| 1.,
        amplitude: &|_| 1.,
        pan: &|_| 0.25,
    });
    let rendered = track.render(4.).samples;
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
        instrument: &Sine,
        hz: &|x| x,
        amplitude: &|_| 1.,
        pan: &|_| 0.5,
    });
    let rendered = track.render(5.).samples;
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
