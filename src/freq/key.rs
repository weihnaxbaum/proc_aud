use super::{semitone, semitone_tet};

/// Calculates the frequency of a note in major scale using
/// [12-tone equal temperament](https://en.wikipedia.org/wiki/12_equal_temperament)
/// based on a
/// [tonic](https://en.wikipedia.org/wiki/Tonic_(music))
/// and a
/// [scale degree](https://en.wikipedia.org/wiki/Degree_(music)).
///
/// # Panics
///
/// If the scale degree isn't in the range 1 to 7.
///
/// # Examples
///
/// ```
/// use proc_aud::prelude::*;
///
/// assert_eq!(maj_key_tet("C4", 1), tet("C4"));
/// assert_eq!(maj_key_tet("C4", 2), tet("D4"));
/// assert_eq!(maj_key_tet("C4", 3), tet("E4"));
/// assert_eq!(maj_key_tet("C4", 4), tet("F4"));
///
/// assert_eq!(maj_key_tet("Db4", 1), tet("Db4"));
/// assert_eq!(maj_key_tet("Db4", 2), tet("Eb4"));
/// assert_eq!(maj_key_tet("Db4", 3), tet("F4"));
/// assert_eq!(maj_key_tet("Db4", 4), tet("Gb4"));
/// ```
pub fn maj_key_tet(tonic: &str, degree: u8) -> f32 {
    semitone_tet(semitone(tonic) + maj_degree_to_semitone(degree))
}

/// Calculates the distance from the tonic in semitones based on a
/// [scale degree](https://en.wikipedia.org/wiki/Degree_(music)) in major scale.
/// The scale degree can range from 1 to 7.
///
/// Major scales follow the scheme 2 whole steps, 1 half step, 3 whole steps, 1 half step.
///
/// # Panics
///
/// If the scale degree isn't in the range 1 to 7.
pub fn maj_degree_to_semitone(degree: u8) -> i32 {
    match degree {
        1 => 0,
        2 => 2,
        3 => 4,
        4 => 5,
        5 => 7,
        6 => 9,
        7 => 11,
        v => panic!("Expected `degree` to be from 1 to 7, found {}", v),
    }
}
