use super::reexports::*;

use std::{process::exit};

pub fn main() {
    let theme = THEME;
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
        items: Items::new(),
        player: RefCell::new(Player::new()),
        npcs: NPCs::new(),
        terrain: terrain,
    });

    let npc0 = globals.npcs.create_npc(Cardinal::North, MoveAI::Hotline(
        Hotline { internal_facing: Cardinal::North }
    ), 8);
    globals.npcs.location_of.fwd().insert(npc0, GlobalPoint {
        r: room,
        x: point2(0, -2),
    });

    globals.at(GlobalPoint {
        r: room,
        x: point2(0, 2)
    }).spawn_item(objdef::ITEM_SHOTGUN.broad());
    globals.at(GlobalPoint {
        r: room,
        x: point2(0, 2)
    }).spawn_item(objdef::ITEM_PISTOL.broad());
    /*
    globals.at(GlobalPoint {
        r: room,
        x: point2(0, 2)
    }).spawn_item(objdef::ITEM_PISTOL.broad());
    */

    Graphics::main_loop(&globals, &mut io);
}

fn test_terrain() -> (Terrain, Id<Room>) {
    let mut terrain = Terrain::new();
    let room0 = terrain.create_room(); 

    for x in -4..=4 {
        for y in -4..=4 {
            terrain.set_block_raw(GlobalPoint { r: room0, x: point2(x, y) }, Block::Empty);
        }
    };

    for x in 10..=18 {
        for y in -4..=4 {
            terrain.set_block_raw(GlobalPoint { r: room0, x: point2(x, y) }, Block::Empty);
        }
    };

    for x in 4..=10 {
        for y in -1..=1 {
            terrain.set_block_raw(GlobalPoint { r: room0, x: point2(x, y) }, Block::Empty);
        }
    };

    for x in 6..=8 {
        for y in -7..=-3 {
            terrain.set_block_raw(GlobalPoint { r: room0, x: point2(x, y) }, Block::Empty);
        }

        for y in 3..=7 {
            terrain.set_block_raw(GlobalPoint { r: room0, x: point2(x, y) }, Block::Empty);
        }
    };

    for x in 4..=12 {
        for y in 9..=13 {
            terrain.set_block_raw(GlobalPoint { r: room0, x: point2(x, y) }, Block::Empty);
        }
    }

    for x in 7..=7 {
        for y in -4..=12 {
            terrain.set_block_raw(GlobalPoint { r: room0, x: point2(x, y) }, Block::Empty);
        }
    }
    terrain.set_block_raw(GlobalPoint { r: room0, x: point2(7, 10) }, Block::Plain);
    terrain.set_block_raw(GlobalPoint { r: room0, x: point2(7, 14) }, Block::Empty);


    terrain.set_player_start_xy(GlobalView {
        r: room0,
        x: point2(0, 0),
        c: Cardinal::North,
    });

    (terrain, room0)
}