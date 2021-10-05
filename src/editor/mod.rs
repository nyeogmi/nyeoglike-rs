use std::{process::exit};

use chiropterm::{*, colors::{Light, LtRed, White}};
use euclid::*;
use moogle::Id;
use crate::{terrain::{Room, Terrain}, widgetry::{Border, Button, Canvas, Column, Deck, InputBox, Label, Row, Scrollable, Spacer, Theme, UI, Window, look_and_feel::WindowBorders}};

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
        editor.io.menu(|_out, _menu| {
        });
    }
}

fn load_file(io: &mut IO) -> Terrain {
    let mut theme = Theme::W95_FRUITY;
    theme.window.borders = WindowBorders::DOS {
        active_title_fg: theme.window.color.1,
        inactive_title_fg: Light[2],
    };

    let ui = UI::new(theme);
    let label: Label = Label::new().setup(|l| {
        l.set_text("Please enter a filename (will be created if the file does not exist). PS Bhijn drinks piss.")
    });
    let prompt1: InputBox = InputBox::new();
    let prompt2: InputBox = InputBox::new();
    let prompt3: InputBox = InputBox::new();
    let prompt4: InputBox = InputBox::new();

    let lbl = label.share();
    let button = Button::new().setup(move |b| {
        b.text = "D - Devour robot".to_owned();
        b.command = Some(Box::new(move |ui, _, _| { 
            let mut l_b = lbl.borrow_mut();
            if l_b.unique.get_text().starts_with("P") {
                l_b.unique.set_text("Nyeh!");
                ui.recompute_layout();
            } else {
                let tx = l_b.unique.get_text().replace("e", "eeeeee"); // unique.text += " Nyeh!"
                l_b.unique.set_text(tx);
                ui.recompute_layout();

                return Signal::Modal(Box::new(|io: &mut IO| {
                    io.menu(|out, menu| {
                        let i = menu.on_click(|_| Signal::Break);
                        out.brush().region(rect(2, 2, 80, 80)).interactor(i, (255, 255)).putfs("HELLO, ROBOT!");
                    });
                    Signal::Continue
                }));
            } 

            Signal::Continue
        }));
    });

    let col: Column = Column::new();
    col.setup(|c| {
        c.add(Spacer::new());
        c.add(label.share());

        c.add(Row::new().setup(|r| {
            r.add(prompt1.setup(|f| f.layout_hacks.expand_horizontally = true).share());
            r.add(prompt2.share());
            r.add(prompt3.share());
            r.add(prompt4.share());
        }));
        c.add(Canvas::new().setup(|c| {
            c.layout_hacks.preferred_width = Some(30);
            c.layout_hacks.preferred_height = Some(2);
            c.set_draw(|b, _| {
                use colors::*;
                b.fill(FSem::new().color((LtRed[2], LtYellow[2])))
            })
        }));
        c.add(button);
        c.add(Spacer::new());
    });

    let win = Window::new();
    win.setup(|w| { 
        w.set_title("TITLE BAR!!!");
        w.set(col.share()) 
    });

    let all0 = Column::new();
    all0.setup(|c| {
        c.add(Spacer::new());
        c.add(win.share());
        c.add(Spacer::new());
        c.add(Window::new().setup(|w|  {
            w.set(Border::new().setup(|b| {
                b.set_north(Label::new().setup(|l| l.set_text("NORTH NORTH NORTH NORTH")));
                b.set_west(Label::new().setup(|l| l.set_text("WEST")));
                b.set_center(Canvas::new().setup(|c| {
                    c.set_draw(|b, _| {
                        b.fill(FSem::new().color((LtRed[1], White)));
                        b.putfs("HELLO, SNACK!!!");
                    });
                    c.layout_hacks.preferred_height = Some(4);
                }));
                b.set_east(Label::new().setup(|l| l.set_text("EAST")));
                b.set_south(Label::new().setup(|l| l.set_text("SOUTH SOUTH SOUTH SOUTH")));
            }))
        }));
        c.add(Spacer::new());
        c.add(Deck::new().setup(|d| {
            d.add(Window::new().setup(|w| w.set_title("WINDOW 1")));
            d.add(Window::new().setup(|w| w.set_title("WINDOW 2")));
            d.add(Window::new().setup(|w| {
                w.set_title("WINDOW 3");
                w.set(Label::new().setup(|l| { l.set_text("I'm a bat!"); }));
            }));
        }));
        c.add(Spacer::new());
    });
    let all = Row::new();
    all.setup(|r| {
        r.add(Spacer::new());
        r.add(all0.share());
        r.add(Spacer::new());
    });

    let all2 = Scrollable::new().setup(|sb| sb.set(all));

    io.menu(|out, menu: Menu| {
        out.brush().fill(FSem::new().color(ui.theme().base.wallpaper));

        all2.draw(ui.share(), out.brush().region(out.rect().inflate(-2, -2)), menu)
    });

    Terrain::new()
}