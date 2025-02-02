use crate::freq::{semitone, semitone_tet};

/// Computes the freqencies in a major triad.
///
/// Output is sorted by freqency.
pub fn maj_triad(root: &str, inversion: i8) -> [f32; 3] {
    triad(root, inversion, 4, 7)
}

/// Computes the freqencies in a minor triad.
///
/// Output is sorted by freqency.
pub fn min_triad(root: &str, inversion: i8) -> [f32; 3] {
    triad(root, inversion, 3, 7)
}

/// Computes the freqencies in a diminished triad.
///
/// Output is sorted by freqency.
pub fn dim_triad(root: &str, inversion: i8) -> [f32; 3] {
    triad(root, inversion, 3, 6)
}

/// Computes the freqencies in an augmented triad.
///
/// Output is sorted by freqency.
pub fn aug_triad(root: &str, inversion: i8) -> [f32; 3] {
    triad(root, inversion, 4, 8)
}

/// Computes the freqencies in a triad.
///
/// Output is sorted by freqency.
/// If you don't need that, use [`triad_unsorted`] instead.
pub fn triad(root: &str, inversion: i8, third_diff: i32, fifth_diff: i32) -> [f32; 3] {
    let mut triad = triad_unsorted(root, inversion, third_diff, fifth_diff);
    triad.sort_by(|a, b| a.total_cmp(b));
    triad
}

/// Computes the freqencies in a triad without sorting by frequency.
///
/// Output is sorted like the following: root, third, fifth.
/// Due to inversions, this may differ from being sorted by frequency.
///
/// If you need the frequencies sorted, use [`triad`] instead.
pub fn triad_unsorted(root: &str, inversion: i8, third_diff: i32, fifth_diff: i32) -> [f32; 3] {
    let root = semitone(root);
    triad_unsorted_semitones(root, inversion, third_diff, fifth_diff)
}

pub(crate) fn triad_unsorted_semitones(
    root: i32,
    inversion: i8,
    third_diff: i32,
    fifth_diff: i32,
) -> [f32; 3] {
    let third = root + third_diff;
    let fifth = root + fifth_diff;
    let mut triad = [root, third, fifth];
    invert_triad_semitones(&mut triad, inversion);
    [
        semitone_tet(triad[0]),
        semitone_tet(triad[1]),
        semitone_tet(triad[2]),
    ]
}

fn invert_triad_semitones(triad: &mut [i32; 3], inversion: i8) {
    match inversion % 3 {
        -2 => {
            triad[1] -= 12;
            triad[2] -= 12;
        }
        -1 => triad[2] -= 12,
        0 => {}
        1 => triad[0] += 12,
        2 => {
            triad[0] += 12;
            triad[1] += 12;
        }
        _ => unreachable!(),
    };
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    // I–V–vi–IV progression in C major
    fn triads() {
        let notes = [
            ["C3", "E3", "G3"],
            ["B2", "D3", "G3"],
            ["C3", "E3", "A3"],
            ["C3", "F3", "A3"],
        ]
        .map(|notes| notes.map(tet));
        let pos_triads = [
            maj_triad("C3", 0),
            maj_triad("G2", 1),
            min_triad("A2", 1),
            maj_triad("F2", 2),
        ];
        let neg_triads = [
            maj_triad("C3", 0),
            maj_triad("G3", -2),
            min_triad("A3", -2),
            maj_triad("F3", -1),
        ];
        let overflow_triads = [
            maj_triad("C3", 3),
            maj_triad("G2", 4),
            min_triad("A3", -5),
            maj_triad("F3", -4),
        ];
        assert_eq!(pos_triads, neg_triads);
        assert_eq!(pos_triads, overflow_triads);
        assert_eq!(pos_triads, notes);
    }
}
