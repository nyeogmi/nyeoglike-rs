use super::reexports::*;

use std::{process::exit};

pub fn main() {
    let theme = Theme::W95_FRUITY;
    let ui = UI::new(theme);
    let mut io = IO::new(
        "Nyeoglike".to_string(), 
        ASPECT_CONFIG, 
        |_| exit(0)
    );

    let terrain = test_terrain();
    let globals = Rc::new(Globals { 
        ui,
        sitemode: Rc::new(RefCell::new(SiteMode::new())),
        terrain: Rc::new(RefCell::new(terrain)),
    });

    main_loop(globals, &mut io);
}

fn main_loop(globals: Rc<Globals>, io: &mut IO) {
    let theme = globals.ui.theme();
    
    let g = globals.clone();
    let sitemode = globals.sitemode.clone();
    let sitemode_display = Canvas::new().setup(|c| {
        c.set_draw(move |brush, menu| {
            sitemode.borrow_mut().draw(&g, brush, menu);
        });
    });

    let sitemode = globals.sitemode.clone();
    io.menu(|out, menu: Menu| {
        // base BG
        out.brush().fill(FSem::new().color(theme.base.wallpaper));

        sitemode.borrow_mut().on_loop(&globals, out.rect());

        sitemode_display.draw(globals.ui.share(), out.brush(), menu)
    });
}

fn test_terrain() -> Terrain {
    let mut terrain = Terrain::new();
    let room = terrain.create_room(); 

    for x in -2..=2 {
        for y in -2..=2 {
            terrain.set(GlobalPoint { r: room, x: point2(x, y) }, Block::Empty);
        }
    }

    terrain.set_player_start_xy(GlobalView {
        r: room,
        x: point2(0, 0),
        c: Cardinal::North,
    });

    terrain
}