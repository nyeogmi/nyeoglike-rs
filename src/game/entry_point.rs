use super::{reexports::*};

use std::{process::exit};

pub fn main() {
    let theme = Theme::W95_FRUITY;
    let ui = UI::new(theme);
    let mut io = IO::new(
        "Nyeoglike".to_string(), 
        ASPECT_CONFIG, 
        |_| exit(0)
    );

    let globals = Globals { 
        ui,
        sitemode: Rc::new(RefCell::new(SiteMode::new())),
    };

    main_loop(globals, &mut io);
}

fn main_loop(globals: Globals, io: &mut IO) {
    let theme = globals.ui.theme();
    let sitemode = globals.sitemode;
    let sitemode_display = Canvas::new().setup(|c| {
        c.set_draw(|brush, menu| {
            // sitemode.borrow_mut().draw(brush, menu);
        });
    });

    io.menu(|out, menu: Menu| {
        out.brush().fill(FSem::new().color(theme.base.wallpaper));

        // win.draw(globals.ui.share(), out.brush().region(out.rect().inflate(-2, -2)), menu)
    });
}