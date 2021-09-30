use std::{process::exit};

use chiropterm::*;
use euclid::*;
use moogle::Id;
use crate::{terrain::{Room, Terrain}, widgetry::{InputBox, UI}};

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
    loop {
        editor.io.menu(|out, menu| {
        });
    }
}

fn load_file(io: &mut IO) -> Terrain {
    use chiropterm::colors::*;

    let prompt = InputBox::new();
    let ui = UI::new();
    loop {
        io.menu(|out, menu| {
            let window = out.brush().region(out.rect().inflate(-2, -2));
            let mut inner = window.region(window.rect().inflate(-1, -1));
            
            out.brush().fill(FSem::new().bg(Green[0]));
            window.fill(FSem::new().bg(Light[2]).fg(Dark[0]));
            window.bevel_w95(Light[3], Dark[0]);

            inner = inner.clone().putfs("Please enter a filename (will be created if the file does not exist):");
            let (above, below) = inner.split_vertically(inner.on_newline().cursor.y);
            prompt.draw(ui.clone(), below, menu)
        });
    }
    Terrain::new()
}