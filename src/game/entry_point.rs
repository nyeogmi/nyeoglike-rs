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
    let globals: Globals = Rc::new(GlobalState { 
        ui,
        sitemode: Rc::new(RefCell::new(SiteMode::new())),
        terrain: Rc::new(RefCell::new(terrain)),
    });

    main_loop(&globals, &mut io);
}

fn main_loop(globals: &Globals, io: &mut IO) {
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

        let g = globals.clone();
        menu.on_tick(move |_| { 
            g.sitemode.borrow_mut().on_tick(&g);
            Signal::Refresh
        });
        sitemode_display.draw(globals.ui.share(), out.brush(), menu)
    });
}

fn test_terrain() -> Terrain {
    let mut terrain = Terrain::new();
    let room0 = terrain.create_room(); 
    let room1 = terrain.create_room(); 
    let room2 = terrain.create_room(); 

    for room in [room0, room1, room2] { 
        for x in -2..=2 {
            for y in -2..=2 {
                terrain.set(GlobalPoint { r: room, x: point2(x, y) }, Block::Empty);
            }
        }
        for x in [-3, 3] {
            terrain.set(GlobalPoint { r: room, x: point2(x, 0) }, Block::Empty);
        }
        for y in [-3, 3] {
            terrain.set(GlobalPoint { r: room, x: point2(0, y) }, Block::Empty);
        }
    }

    terrain.set_player_start_xy(GlobalView {
        r: room0,
        x: point2(0, 0),
        c: Cardinal::North,
    });

    terrain.add_area_portal(AreaPortal {
        src: GlobalView { r: room0, x: point2(0, -3), c: Cardinal::North},
        dst: GlobalView { r: room1, x: point2(-3, 0), c: Cardinal::East},
        size: 1,
    });

    terrain
}