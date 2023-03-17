type NoteCollection = Vec<u8>;

#[derive(Debug)]
struct Scale {
    notes: NoteCollection,
}

impl Scale {
    fn new(notes: NoteCollection) -> Scale {
        Scale { notes }
    }
}

fn main() {
    let major = Scale::new(vec!(0, 2, 4, 5, 7, 9, 11));
    println!("This is the major scale: {:?}", major);
}
