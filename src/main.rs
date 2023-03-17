use std::collections::HashSet;

type NoteCollection = Vec<i8>;

fn rotate(notes: NoteCollection) -> NoteCollection {
    let shift_amount = notes[1];
    let mut shifted_notes = notes
        .iter()
        .map(|note| (note - shift_amount) % 12)
        .map(|note| if note < 0 { note + 12 } else { note } )
        .collect::<NoteCollection>();
    shifted_notes.sort();
    shifted_notes
}

fn note_collection_distance(a: &NoteCollection, b: &NoteCollection) -> i8 {
    if a.len() != b.len() {
        panic!("Cannot calculate distance between note collections of different lengths");
    }
    a.iter().zip(b.iter()).map(|(a_item, b_item)| (a_item - b_item).abs()).sum()
}

#[derive(Debug)]
struct Scale {
    notes: NoteCollection,
    modes: HashSet<NoteCollection>,
}

impl Scale {
    fn new(notes: NoteCollection) -> Scale {
        let mut modes = HashSet::new();
        let mut mode = notes.clone();
        modes.insert(mode.clone());
        loop {
            mode = rotate(mode);
            if mode == notes {
                break;
            }
            modes.insert(mode.clone());
        }
        Scale { notes, modes }
    }

    fn equals(&self, other: &Scale) -> bool {
        self.notes == other.notes
    }

    fn is_mode_of(&self, other: &Scale) -> bool {
        self.modes.intersection(&other.modes).count() > 0
    }

    fn distance_from(&self, other: &Scale) -> i8 {
        if let Some(distance) = self.modes.iter().map(
            |mode| note_collection_distance(mode, &other.notes)
        ).min() {
            return distance;
        };
        0
    }
}

fn main() {
    let major = Scale::new(vec!(0, 2, 4, 5, 7, 9, 11));
    let melodic_minor = Scale::new(vec!(0, 2, 3, 5, 7, 9, 11));
    let hungarian_minor = Scale::new(vec!(0, 2, 3, 6, 7, 8, 11));
    for scale in [&major, &melodic_minor] {
        println!(
            "Distance to Hungarian Minor: {:?} = {}",
            scale,
            hungarian_minor.distance_from(scale),
        );
    }
    major.distance_from(&melodic_minor);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scale_modes_generated_correctly() {
        let major = Scale::new(vec!(0, 2, 4, 5, 7, 9, 11));
        assert_eq!(
            major.modes,
            HashSet::from([
                vec!(0, 2, 4, 5, 7, 9, 11),
                vec!(0, 2, 3, 5, 7, 9, 10),
                vec!(0, 1, 3, 5, 7, 8, 10),
                vec!(0, 2, 4, 6, 7, 9, 11),
                vec!(0, 2, 4, 5, 7, 9, 10),
                vec!(0, 2, 3, 5, 7, 8, 10),
                vec!(0, 1, 3, 5, 6, 8, 10),
            ])
        );
    }

    #[test]
    fn test_scale_equals_itself() {
        let major = Scale::new(vec!(0, 2, 4, 5, 7, 9, 11));
        assert!(major.equals(&major));
    }

    #[test]
    fn test_dorian_is_mode_of_major() {
        let major = Scale::new(vec!(0, 2, 4, 5, 7, 9, 11));
        let dorian = Scale::new(vec!(0, 2, 3, 5, 7, 9, 10));
        assert!(major.is_mode_of(&dorian));
        assert!(dorian.is_mode_of(&major));
    }

    #[test]
    fn test_melodic_minor_is_not_major() {
        let major = Scale::new(vec!(0, 2, 4, 5, 7, 9, 11));
        let melodic_minor = Scale::new(vec!(0, 2, 3, 5, 7, 9, 11));
        assert!(!major.equals(&melodic_minor));
        assert!(!major.is_mode_of(&melodic_minor));
    }

    #[test]
    fn test_scale_distance() {
        let major = Scale::new(vec!(0, 2, 4, 5, 7, 9, 11));
        let melodic_minor = Scale::new(vec!(0, 2, 3, 5, 7, 9, 11));
        assert_eq!(major.distance_from(&major), 0);
        assert_eq!(major.distance_from(&melodic_minor), 1);
    }
}
