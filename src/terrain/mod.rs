mod block;
mod level;
mod room;

use moogle::{Id, Pom};

pub use block::Block;
pub use level::Level;
pub use room::Room;

use crate::geom::*;

pub struct Terrain {
    player_start_xy: Option<GlobalView>,
    portals: Portals,

    // levels: Pom<Level>,
    rooms: Pom<Room>,

    // level_rooms: OneToMany<Id<Level>, Id<Room>>,
}

impl Terrain {
    pub fn new() -> Terrain {
        Terrain { 
            player_start_xy: None,
            portals: Portals::new(),
            // levels: Pom::new(), 
            rooms: Pom::new(), 

            // level_rooms: OneToMany::new(), 
        }
    }

    pub fn recalculate_egosphere(&self, egosphere: &mut Egosphere, viewport: Viewport) {
        egosphere.calculate(
            viewport,
            &self.portals,
            |gv| {
                self.get(gv.point()).is_blocked()
            }
        );
    }

    pub fn set(&mut self, gp: GlobalPoint, b: Block) {
        if let Some(r) = self.rooms.get_mut(gp.r) {
            r.set(gp.x.cast_unit(), b);
        } else {
            panic!("invalid room ID");
        }
    }

    pub fn get(&self, gp: GlobalPoint) -> Block {
        if let Some(r) = self.rooms.get(gp.r) {
            return r.get(gp.x.cast_unit())
        };
        Block::Plain
    }

    pub fn set_player_start_xy(&mut self, player_start_xy: GlobalView) {
        self.player_start_xy = Some(player_start_xy);
    }

    pub fn suggest_player_xy(&self) -> Option<GlobalView> {
        self.player_start_xy
    }

    pub(crate) fn create_room(&mut self) -> Id<Room> {
        self.rooms.insert(Room::new())
    }
}