use crate::chord::QUALITIES;
use crate::interval::Interval;
use crate::note::Note;

/// Returns chord symbols whose pitch-class sets exactly match the input.
pub fn identify(notes: &[Note]) -> Vec<String> {
    if notes.is_empty() {
        return Vec::new();
    }

    let mut input_pitch_classes = [false; 12];
    for note in notes {
        input_pitch_classes[note.pc() as usize] = true;
    }

    let first_note = notes[0];
    let mut seen_roots = [false; 12];
    let mut matches = Vec::new();

    for root in notes {
        let root_pc = root.pc() as usize;
        if seen_roots[root_pc] {
            continue;
        }
        seen_roots[root_pc] = true;

        for (quality, degrees) in QUALITIES {
            let mut chord_pitch_classes = [false; 12];
            let mut valid_quality = true;

            for token in degrees.split_ascii_whitespace() {
                let Some(degree) = Interval::degree(token) else {
                    valid_quality = false;
                    break;
                };
                let pitch_class = (root.pc() + degree.semis).rem_euclid(12) as usize;
                chord_pitch_classes[pitch_class] = true;
            }

            if valid_quality && chord_pitch_classes == input_pitch_classes {
                let mut name = format!("{}{}", root.name(), quality);
                if first_note.pc() != root.pc() {
                    name.push('/');
                    name.push_str(&first_note.name());
                }
                matches.push(name);
            }
        }
    }

    matches
}
