use std::{process::exit};

use chiropterm::*;
use euclid::size2;
use crate::widgetry::*;

const ASPECT_CONFIG: AspectConfig = AspectConfig {
    pref_min_term_size: size2(80, 50),  // but expect ~112x60
    pref_max_term_size: size2(256, 256),
};

pub fn main() {
    let theme = Theme::W95_FRUITY;
    let ui = UI::new(theme);
    let mut io = IO::new(
        "Nyeoglike".to_string(), 
        ASPECT_CONFIG, 
        |_| exit(0)
    );

    main_loop(ui.share(), &mut io);
}

fn main_loop(ui: UI, io: &mut IO) {
    let win = Window::new();
    win.setup(|w| { 
        w.set_title("TITLE BAR!!!");
        w.set(Label::new().setup(|l| {
            l.set_text("Test label!")
        })) 
    });

    io.menu(|out, menu: Menu| {
        out.brush().fill(FSem::new().color(ui.theme().base.wallpaper));

        win.draw(ui.share(), out.brush().region(out.rect().inflate(-2, -2)), menu)
    });
}