use std::process::exit;

use chiropterm::*;
use euclid::*;
use moogle::Id;
use crate::terrain::{Room, Terrain};

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
        editor.io.menu_nil(|out, menu| {
        });
    }
}

fn load_file(mut io: &mut IO) -> Terrain {
    let mut prompt = PromptBox::new();
    loop {
        let cmd = io.menu(|out, menu| {
            let b = out.brush().putfs("Please enter a filename (will be created if does not exist):");

            let (_, below) = b.split_vertically(b.on_newline().cursor.y);
            prompt.draw(below, menu)
        });
        prompt.handle(cmd)
    }
    Terrain::new()
}

struct PromptBox {
    text: String,
    text_i: usize,
}

impl PromptBox {
    fn new() -> PromptBox {
        PromptBox { text: "fuck you lamers".to_string(), text_i: 0 }
    }

    fn draw<T: Brushable>(&self, brush: Brush<T>, menu: &Menu<PromptBoxCommand>) {
        let interactor = menu.on(Keycode::A, |_| PromptBoxCommand::IncrI(1));
        menu.on_key(Keycode::Backspace, |_| PromptBoxCommand::IncrI(-1));
        brush.interactor(interactor).putfs(&self.text[..self.text_i]);
    }

    fn handle(&mut self, cmd: PromptBoxCommand) {
        match cmd {
            PromptBoxCommand::IncrI(i) => { 
                let mut ti2 = self.text_i as isize + i;
                if ti2 < 0 { ti2 = 0 }
                if ti2 >= self.text.len() as isize { ti2 = self.text.len() as isize }
                self.text_i = ti2 as usize;
            }
        }
    }
}

enum PromptBoxCommand {
    IncrI(isize),
}