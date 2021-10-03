use std::{process::exit};

use chiropterm::*;
use euclid::*;
use moogle::Id;
use crate::{terrain::{Room, Terrain}, widgetry::{Button, Column, InputBox, Label, Theme, UI, Window, look_and_feel::WindowBorders}};

const ASPECT_CONFIG: AspectConfig = AspectConfig {
    pref_min_term_size: size2(80, 50),  // but expect ~112x60
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

    let mut theme = Theme::W95_FRUITY;
    theme.window.borders = WindowBorders::DOS {};

    let ui = UI::new(theme);
    let label: Label<()> = Label::new().setup(|l| {
        l.text = "Please enter a filename (will be created if the file does not exist). PS Bhijn drinks piss.".to_string()
    });
    let prompt1: InputBox<()> = InputBox::new();
    let prompt2: InputBox<()> = InputBox::new();
    let prompt3: InputBox<()> = InputBox::new();
    let prompt4: InputBox<()> = InputBox::new();

    let lbl = label.share();
    let button = Button::new().setup(move |b| {
        b.text = "D - Devour robot".to_owned();
        b.command = Some(Box::new(move |ui, _, _| { 
            let mut l_b = lbl.borrow_mut();
            if l_b.unique.text.starts_with("P") {
                l_b.unique.text = "Nyeh!".to_owned();
            } else {
                // l_b.unique.text += " Nyeh!"
                l_b.unique.text = l_b.unique.text.replace("e", "ee"); // unique.text += " Nyeh!"
            } 
            ui.recompute_layout();
            Signal::Continue
        }));
    });

    let col: Column<()> = Column::new();
    col.setup(|c| {
        c.add(label.share());
        c.add(prompt1.share());
        c.add(prompt2.share());
        c.add(prompt3.share());
        c.add(prompt4.share());
        c.add(button);
    });

    let win = Window::new();
    win.setup(|w| { w.set_widget(col.share()) });

    io.menu(|out, menu: Menu<()>| {
        // let mut window = out.brush().region(out.rect().inflate(-2, -2));
        // window = window.reshape_for_font();
        
        out.brush().fill(FSem::new().color(ui.theme().base.wallpaper));

        /*
        window.fill(FSem::new().color(ui.theme().window.color));
        window.bevel_w95(ui.theme().window.bevel);

        window.font(Font::Normal).draw_box(true);
        window.draw_box(true);
        */

        // TODO: Use labels or something for this bit, when they're implemented
        win.draw(ui.share(), out.brush().region(out.rect().inflate(-2, -2)), menu)
    });

    Terrain::new()
}