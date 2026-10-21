pub const NATURAL: [i32; 7] = [0, 2, 4, 5, 7, 9, 11];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Note {
    pub letter: u8,
    pub acc: i8,
}

impl Note {
    pub fn parse(text: &str) -> Option<Note> {
        let bytes = text.as_bytes();

        match bytes {
            [letter] => Some(Note {
                letter: letter_index(*letter)?,
                acc: 0,
            }),
            [letter, b'#'] => Some(Note {
                letter: letter_index(*letter)?,
                acc: 1,
            }),
            [letter, b'#', b'#'] => Some(Note {
                letter: letter_index(*letter)?,
                acc: 2,
            }),
            [letter, b'b'] => Some(Note {
                letter: letter_index(*letter)?,
                acc: -1,
            }),
            [letter, b'b', b'b'] => Some(Note {
                letter: letter_index(*letter)?,
                acc: -2,
            }),
            _ => None,
        }
    }

    pub fn name(&self) -> String {
        let letters = ['C', 'D', 'E', 'F', 'G', 'A', 'B'];
        let letter = letters
            .get(usize::from(self.letter))
            .copied()
            .unwrap_or('?');
        let accidental = accidental_symbol(self.acc < 0);
        let mut name = String::with_capacity(1 + usize::from(self.acc.unsigned_abs()));
        name.push(letter);
        for _ in 0..self.acc.unsigned_abs() {
            name.push(accidental);
        }
        name
    }

    pub fn pc(&self) -> i32 {
        let natural = NATURAL.get(usize::from(self.letter)).copied().unwrap_or(0);
        (natural + i32::from(self.acc)).rem_euclid(12)
    }
}

#[cfg(not(feature = "unicode"))]
fn accidental_symbol(flat: bool) -> char {
    if flat { 'b' } else { '#' }
}

#[cfg(feature = "unicode")]
fn accidental_symbol(flat: bool) -> char {
    if flat { '\u{266D}' } else { '\u{266F}' }
}

fn letter_index(letter: u8) -> Option<u8> {
    match letter {
        b'C' => Some(0),
        b'D' => Some(1),
        b'E' => Some(2),
        b'F' => Some(3),
        b'G' => Some(4),
        b'A' => Some(5),
        b'B' => Some(6),
        _ => None,
    }
}
