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
        graphics: RefCell::new(Graphics::new()),
        player: RefCell::new(Player::new()),
        terrain: Rc::new(RefCell::new(terrain)),
    });

    main_loop(&globals, &mut io);
}

fn main_loop(globals: &Globals, io: &mut IO) {
    let theme = globals.ui.theme();
    
    let g = globals.clone();
    let sitemode_display = Canvas::new().setup(|c| {
        c.set_draw(move |brush, menu| {
            g.graphics.borrow_mut().draw(&g, brush, menu);
        });
    });

    let g = globals.clone();
    io.menu(|out, menu: Menu| {
        // base BG
        out.brush().fill(FSem::new().color(theme.base.wallpaper));

        let g = g.clone();
        let rect = out.rect();
        menu.on_tick(move |_| { 
            // TODO: Figure out if the rect was resized and if so, also call pre_tick_or_resize?
            g.graphics.borrow_mut().pre_tick_or_resize(&g, rect);
            g.player.borrow_mut().on_tick(&g);
            g.graphics.borrow_mut().post_tick_or_resize(&g, rect);
            Signal::Refresh
        });
        sitemode_display.draw(globals.ui.share(), out.brush(), menu)
    });
}

/* 
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

    for y in [-4, 4] {
        terrain.set(GlobalPoint { r: room1, x: point2(0, y) }, Block::Empty);
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

    terrain.add_area_portal(AreaPortal {
        src: GlobalView { r: room1, x: point2(0, -4), c: Cardinal::East},
        dst: GlobalView { r: room1, x: point2(0, 4), c: Cardinal::East},
        size: 1,
    });

    terrain.add_area_portal(AreaPortal {
        src: GlobalView { r: room1, x: point2(3, 0), c: Cardinal::East},
        dst: GlobalView { r: room2, x: point2(0, 3), c: Cardinal::North},
        size: 1,
    });

    terrain.add_area_portal(AreaPortal {
        src: GlobalView { r: room2, x: point2(0, -3), c: Cardinal::North},
        dst: GlobalView { r: room0, x: point2(-3, 0), c: Cardinal::East},
        size: 1,
    });

    terrain
}
*/

fn test_terrain() -> Terrain {
    let mut terrain = Terrain::new();
    let room0 = terrain.create_room(); 

    for x in -4..=4 {
        for y in -4..=4 {
            terrain.set(GlobalPoint { r: room0, x: point2(x, y) }, Block::Empty);
        }
    };

    for x in 10..=14 {
        for y in -4..=4 {
            terrain.set(GlobalPoint { r: room0, x: point2(x, y) }, Block::Empty);
        }
    };

    for x in 4..=10 {
        for y in -1..=1 {
            terrain.set(GlobalPoint { r: room0, x: point2(x, y) }, Block::Empty);
        }
    };

    terrain.set_player_start_xy(GlobalView {
        r: room0,
        x: point2(0, 0),
        c: Cardinal::North,
    });

    terrain
}