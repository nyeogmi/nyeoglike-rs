use crate::game::reexports::*;

use super::{Behavior, graphics::Memory};

pub struct SiteMode {
    // behavior
    pub(super) player_xy: Option<GlobalView>,
    pub(super) behavior: Behavior,

    // graphics
    pub(super) egosphere: Egosphere,
    pub(super) memory: Memory,
}

impl SiteMode {
    pub fn new() -> SiteMode {
        SiteMode {
            player_xy: None,
            behavior: Behavior::new(),

            egosphere: Egosphere::new(false),
            memory: Memory::new(),
        }
    }

    pub fn move_by(&mut self, globals: &Globals, offset: EgoVec) -> EgoVec {
        let terrain: Ref<Terrain> = globals.terrain.borrow();
        if let Some(player_xy) = self.player_xy {
            let mut offset_x0 = offset; offset_x0.x = 0;
            let mut offset_y0 = offset; offset_y0.y = 0;
            for (i, &thing_to_try) in [offset, offset_x0, offset_y0].iter().enumerate() {
                if thing_to_try == offset && i != 0 {
                    continue;  // don't try offset more than once
                }

                let new_xy = terrain.step_offset(player_xy, thing_to_try);
                let at_new_xy = terrain.get(new_xy.point());
                if at_new_xy.is_blocked() {
                    continue;
                }

                self.player_xy = self.player_xy.map(|pxy| terrain.step_offset(pxy, thing_to_try));
                self.shift_memory(-thing_to_try);
                return thing_to_try;
            }
            return EgoVec::zero();
        } else {
            return EgoVec::zero();
        }
    }
}