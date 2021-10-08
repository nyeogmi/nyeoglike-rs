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

    let (terrain, room) = test_terrain();
    let globals: Globals = Rc::new(GlobalState { 
        ui,
        graphics: RefCell::new(Graphics::new()),
        player: RefCell::new(Player::new()),
        npcs: RefCell::new(NPCs::new()),
        terrain: RefCell::new(terrain),
    });

    let npc0 = globals.npcs.borrow_mut().create_npc(Cardinal::North, MoveAI::Hotline(
        Hotline { internal_facing: Cardinal::North }
    ), 8);
    globals.npcs.borrow().location_of.fwd().insert(npc0, GlobalPoint {
        r: room,
        x: point2(0, -2),
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
            // update graphics
            g.graphics.borrow_mut().pre_tick_or_resize(&g, rect);

            g.npcs.borrow_mut().pre_tick(&g);
            g.player.borrow_mut().on_tick(&g);
            g.npcs.borrow_mut().tick(&g);

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

fn test_terrain() -> (Terrain, Id<Room>) {
    let mut terrain = Terrain::new();
    let room0 = terrain.create_room(); 

    for x in -4..=4 {
        for y in -4..=4 {
            terrain.set(GlobalPoint { r: room0, x: point2(x, y) }, Block::Empty);
        }
    };

    for x in 10..=18 {
        for y in -4..=4 {
            terrain.set(GlobalPoint { r: room0, x: point2(x, y) }, Block::Empty);
        }
    };

    for x in 4..=10 {
        for y in -1..=1 {
            terrain.set(GlobalPoint { r: room0, x: point2(x, y) }, Block::Empty);
        }
    };

    for x in 6..=8 {
        for y in -7..=-3 {
            terrain.set(GlobalPoint { r: room0, x: point2(x, y) }, Block::Empty);
        }

        for y in 3..=7 {
            terrain.set(GlobalPoint { r: room0, x: point2(x, y) }, Block::Empty);
        }
    };

    for x in 4..=12 {
        for y in 9..=13 {
            terrain.set(GlobalPoint { r: room0, x: point2(x, y) }, Block::Empty);
        }
    }

    for x in 7..=7 {
        for y in -4..=12 {
            terrain.set(GlobalPoint { r: room0, x: point2(x, y) }, Block::Empty);
        }
    }
    terrain.set(GlobalPoint { r: room0, x: point2(7, 10) }, Block::Plain);
    terrain.set(GlobalPoint { r: room0, x: point2(7, 14) }, Block::Empty);


    terrain.set_player_start_xy(GlobalView {
        r: room0,
        x: point2(0, 0),
        c: Cardinal::North,
    });

    (terrain, room0)
}