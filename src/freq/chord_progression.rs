use super::{
    key::{maj_degree_to_semitone, min_degree_to_semitone},
    semitone,
    triad::triad_unsorted_semitones,
};

/// Calculates the frequencies of a chord progression in major scale. Result consists of triads.
///
/// Chord progressions are expressed by roman numerals ranging from 1 to 7.
/// Each numeral represents the root of one chord.
/// Chords are separated by dashes ('-').
///
/// Frequencies in the triads are sorted. For unsorted triads, see
/// [`maj_chord_progression_unsorted`].
///
/// # Panics
///
/// If `inversions.len()` != length of chord progression.
///
/// # Examples
///
/// ```
/// use proc_aud::prelude::*;
///
/// let progression = maj_chord_progression("C4", "I-ii-V-i", &[2, 1, 0, 0]);
/// assert_eq!(progression[0], maj_triad("C4", 2));
/// assert_eq!(progression[1], min_triad("D4", 1));
/// assert_eq!(progression[2], maj_triad("G4", 0));
/// assert_eq!(progression[3], min_triad("C4", 0));
/// ```
pub fn maj_chord_progression(tonic: &str, progression: &str, inversions: &[i8]) -> Vec<[f32; 3]> {
    sort(maj_chord_progression_unsorted(
        tonic,
        progression,
        inversions,
    ))
}

/// Calculates the frequencies of a chord progression in major scale. Result consists of triads.
///
/// Chord progressions are expressed by roman numerals ranging from 1 to 7.
/// Each numeral represents the root of one chord.
/// Chords are separated by dashes ('-').
///
/// # Panics
///
/// If `inversions.len()` != length of chord progression.
///
/// # Examples
///
/// ```
/// use proc_aud::freq::{
///     chord_progression::maj_chord_progression_unsorted,
///     triad::triad_unsorted,
/// };
///
/// let progression = maj_chord_progression_unsorted("C4", "I-ii-V-i", &[2, 1, 0, 0]);
/// assert_eq!(progression[0], triad_unsorted("C4", 2, 4, 7)); // major chord
/// assert_eq!(progression[1], triad_unsorted("D4", 1, 3, 7)); // minor chord
/// assert_eq!(progression[2], triad_unsorted("G4", 0, 4, 7)); // major chord
/// assert_eq!(progression[3], triad_unsorted("C4", 0, 3, 7)); // minor chord
/// ```
pub fn maj_chord_progression_unsorted(
    tonic: &str,
    progression: &str,
    inversions: &[i8],
) -> Vec<[f32; 3]> {
    chord_progression_unsorted(tonic, progression, inversions, maj_degree_to_semitone)
}

/// Calculates the frequencies of a chord progression in minor scale. Result consists of triads.
///
/// Chord progressions are expressed by roman numerals ranging from 1 to 7.
/// Each numeral represents the root of one chord.
/// Chords are separated by dashes ('-').
///
/// Frequencies in the triads are sorted. For unsorted triads, see
/// [`min_chord_progression_unsorted`].
///
/// # Panics
///
/// If `inversions.len()` != length of chord progression.
///
/// # Examples
///
/// ```
/// use proc_aud::prelude::*;
///
/// let progression = min_chord_progression("A4", "I-ii-V-i", &[2, 1, 0, 0]);
/// assert_eq!(progression[0], maj_triad("A4", 2));
/// assert_eq!(progression[1], min_triad("B4", 1));
/// assert_eq!(progression[2], maj_triad("E5", 0));
/// assert_eq!(progression[3], min_triad("A4", 0));
/// ```
pub fn min_chord_progression(tonic: &str, progression: &str, inversions: &[i8]) -> Vec<[f32; 3]> {
    sort(min_chord_progression_unsorted(
        tonic,
        progression,
        inversions,
    ))
}

/// Calculates the frequencies of a chord progression in minor scale. Result consists of triads.
///
/// Chord progressions are expressed by roman numerals ranging from 1 to 7.
/// Each numeral represents the root of one chord.
/// Chords are separated by dashes ('-').
///
/// # Panics
///
/// If `inversions.len()` != length of chord progression.
///
/// # Examples
///
/// ```
/// use proc_aud::freq::{
///     chord_progression::min_chord_progression_unsorted,
///     triad::triad_unsorted,
/// };
///
/// let progression = min_chord_progression_unsorted("A4", "I-ii-V-i", &[2, 1, 0, 0]);
/// assert_eq!(progression[0], triad_unsorted("A4", 2, 4, 7)); // major chord
/// assert_eq!(progression[1], triad_unsorted("B4", 1, 3, 7)); // minor chord
/// assert_eq!(progression[2], triad_unsorted("E5", 0, 4, 7)); // major chord
/// assert_eq!(progression[3], triad_unsorted("A4", 0, 3, 7)); // minor chord
/// ```
pub fn min_chord_progression_unsorted(
    tonic: &str,
    progression: &str,
    inversions: &[i8],
) -> Vec<[f32; 3]> {
    chord_progression_unsorted(tonic, progression, inversions, min_degree_to_semitone)
}

fn sort(mut progression: Vec<[f32; 3]>) -> Vec<[f32; 3]> {
    for triad in &mut progression {
        triad.sort_by(|a, b| a.total_cmp(b));
    }
    progression
}

fn chord_progression_unsorted(
    tonic: &str,
    progression: &str,
    inversions: &[i8],
    degree_to_semitone: impl Fn(u8) -> i32,
) -> Vec<[f32; 3]> {
    let mut result = vec![];
    for (i, s) in progression.split('-').enumerate() {
        let third_diff = get_third_diff(s);
        let degree = parse_degree(s);
        let semitone = semitone(tonic) + degree_to_semitone(degree);
        let triad = triad_unsorted_semitones(semitone, inversions[i], third_diff, 7);
        result.push(triad);
    }
    result
}

fn get_third_diff(degree: &str) -> i32 {
    if degree.chars().next().unwrap().is_uppercase() {
        4
    } else {
        3
    }
}

fn parse_degree(degree: &str) -> u8 {
    match degree.to_lowercase().as_str() {
        "i" => 1,
        "ii" => 2,
        "iii" => 3,
        "iv" => 4,
        "v" => 5,
        "vi" => 6,
        "vii" => 7,
        degree => panic!("Expected roman numeral from 1 to 7, found {}", degree),
    }
}
