use std::{cell::RefCell, process::exit, rc::Rc};

use chiropterm::*;
use euclid::*;
use moogle::Id;
use crate::{terrain::{Room, Terrain}, widgetry::{PromptBox, PromptBoxState}};

const ASPECT_CONFIG: AspectConfig = AspectConfig {
    pref_min_term_size: size2(64, 48),  // but expect ~112x60
    pref_max_term_size: size2(256, 256),
};

struct EditorState {
    io: IO, 

    terrain: Terrain,
    room: Option<Id<Room>>,
    cursor: Point2D<isize, isize>
}

pub fn main() {
    // TODO: Load terrain from disk, if present
    let mut io = IO::new(
        "Nyeoglike editor".to_string(), 
        ASPECT_CONFIG, 
        |_| exit(0)
    );

    let terrain = load_file(&mut io);

    main_loop(EditorState {
        io, 

        terrain,
        room: None,
        cursor: point2(0, 0),
    })
}

fn main_loop(mut editor: EditorState) {
    use chiropterm::colors::*;

    loop {
        editor.io.menu(|out, menu| {
        });
    }
}

fn load_file(io: &mut IO) -> Terrain {
    let prompt = PromptBox::new();
    loop {
        io.menu(|out, menu| {
            let b = out.brush().putfs("Please enter a filename (will be created if does not exist):");

            let (_, below) = b.split_vertically(b.on_newline().cursor.y);
            prompt.draw(below, menu)
        });
    }
    Terrain::new()
}