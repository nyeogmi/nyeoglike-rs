use std::{process::exit};

use chiropterm::*;
use euclid::*;
use moogle::Id;
use crate::{terrain::{Room, Terrain}, widgetry::{Column, InputBox, UI}};

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
        editor.io.menu(|_out, _menu: Menu<()>| {
        });
    }
}

fn load_file(io: &mut IO) -> Terrain {
    use chiropterm::colors::*;

    let ui = UI::new();
    let prompt1: InputBox<()> = InputBox::new();
    let prompt2: InputBox<()> = InputBox::new();
    let prompt3: InputBox<()> = InputBox::new();
    let prompt4: InputBox<()> = InputBox::new();

    let col: Column<()> = Column::new();
    col.borrow_mut().unique.add(prompt1.share());
    col.borrow_mut().unique.add(prompt2.share());
    col.borrow_mut().unique.add(prompt3.share());
    col.borrow_mut().unique.add(prompt4.share());

    io.menu(|out, menu| {
        let window = out.brush().region(out.rect().inflate(-2, -2));
        let mut inner = window.region(window.rect().inflate(-1, -1));
        
        out.brush().fill(FSem::new().bg(Green[0]));
        window.fill(FSem::new().bg(Light[2]).fg(Dark[0]));
        window.bevel_w95(Light[3], Dark[0]);

        inner = inner.clone().putfs("Please enter a filename (will be created if the file does not exist):");
        let (_above, below) = inner.split_vertically(inner.on_newline().cursor.y);
        col.draw(ui.share(), below, menu)
    });

    Terrain::new()
}