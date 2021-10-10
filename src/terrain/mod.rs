mod block;
mod level;
mod room;

use crate::reexports::*;

pub use block::*;
pub use level::*;
pub use room::*;

pub struct Terrain {
    player_start_xy: Option<GlobalView>,
    portals: Portals,

    // levels: FloatingPom<Level>,
    rooms: FloatingPom<Room>,  // we don't need the iterator-friendly features of Pom, but do need to be able to create rooms on the fly

    // level_rooms: OneToMany<Id<Level>, Id<Room>>,
}

impl Terrain {
    pub fn new() -> Terrain {
        Terrain { 
            player_start_xy: None,
            portals: Portals::new(),
            // levels: FloatingPom::new(), 
            rooms: FloatingPom::new(),

            // level_rooms: OneToMany::new(), 
        }
    }

    pub fn recalculate_egosphere(&self, egosphere: &mut Egosphere, viewport: Viewport, blocked: impl Fn(GlobalView) -> bool) {
        egosphere.calculate(viewport, &self.portals, blocked);
    }

    pub fn set_block_raw(&self, gp: GlobalPoint, b: Block) {
        if let Some(r) = self.rooms.get(gp.r) {
            r.borrow().set(gp.x.cast_unit(), b);
        } else {
            panic!("invalid room ID");
        }
    }

    pub fn get_block_raw(&self, gp: GlobalPoint) -> Block {
        if let Some(r) = self.rooms.get(gp.r) {
            return r.borrow().get(gp.x.cast_unit())
        };
        Block::Plain
    }

    pub fn set_player_start_xy(&mut self, player_start_xy: GlobalView) {
        self.player_start_xy = Some(player_start_xy);
    }

    pub fn suggest_player_xy(&self) -> Option<GlobalView> {
        self.player_start_xy
    }

    pub fn create_room(&mut self) -> Id<Room> {
        self.rooms.insert(Room::new())
    }

    pub fn add_area_portal(&mut self, area_portal: AreaPortal) {
        self.portals.add_area_portal(area_portal)
    }

    pub fn step_offset(&self, point: GlobalView, offset: EgoVec) -> GlobalView {
        self.portals.step_offset(point, offset).1
    }
}