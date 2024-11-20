pub const A4: f32 = 440.;
const A4_SEMITONE: i32 = 57;

pub fn tet(note: &str) -> f32 {
    let semitone = semitone(note);
    let diff = semitone - A4_SEMITONE;
    A4 * 2.0f32.powf(1. / 12.).powi(diff)
}

fn semitone(note: &str) -> i32 {
    let note = note.to_ascii_lowercase();
    let mut chars = note.chars();

    let letter = match chars.next().unwrap() {
        'c' => 0,
        'd' => 2,
        'e' => 4,
        'f' => 5,
        'g' => 7,
        'a' => 9,
        'b' => 11,
        c => panic!("Expected a note name from A to G, recieved {c}"),
    };

    let char2 = chars.next().unwrap();

    let modifier = match char2 {
        '#' => 1,
        'b' | '♭' => -1,
        _ => 0,
    };

    let octave = if modifier == 0 {
        char2
    } else {
        chars.next().unwrap()
    }
    .to_digit(10)
    .unwrap() as i32
        * 12;

    letter + modifier + octave
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn semitones() {
        assert_eq!(semitone("C0"), 0);
        assert_eq!(semitone("c0"), 0);
        assert_eq!(semitone("Cb0"), -1);
        assert_eq!(semitone("C♭0"), -1);
        assert_eq!(semitone("C1"), 12);
        assert_eq!(semitone("C#1"), 13);
        assert_eq!(semitone("A4"), 57);
        assert_eq!(semitone("b#9"), 120);
    }

    #[test]
    fn tet_frequencies() {
        assert_approx_eq(tet("cb0"), 15.43385);
        assert_approx_eq(tet("c0"), 16.3516);
        assert_approx_eq(tet("c3"), 130.8128);
        assert_approx_eq(tet("e3"), 164.8138);
        assert_eq!(tet("A4"), 440.);
        assert_approx_eq(tet("f#6"), 1479.978);
        assert_approx_eq(tet("G♭6"), 1479.978);
        assert_approx_eq(tet("b8"), 7902.133);
    }

    fn assert_approx_eq(left: f32, right: f32) {
        let epsilon = 0.02;
        assert!(
            (left - right).abs() < epsilon,
            "left: {} != right: {}",
            left,
            right,
        );
    }
}
