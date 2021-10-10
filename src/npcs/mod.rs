mod hotline;

use crate::reexports::*;
pub use hotline::Hotline;

pub struct NPCs {
    pub table: FloatingPom<NPC>, 

    // TODO: Don't let people use this directly (so we can force an egosphere recalc)
    pub location_of: ManyToOne<Id<NPC>, GlobalPoint>,
}

impl NPCs {
    pub fn new() -> NPCs {
        NPCs {  
            table: FloatingPom::new(),
            location_of: ManyToOne::new(),
        }
    }

    pub fn create_npc(&self, facing: Cardinal, move_ai: MoveAI, view_radius: isize) -> Id<NPC> {
        self.table.insert(NPC {
            facing,
            move_ai,
            view_radius,
            egosphere: Egosphere::new(false),
        })
    }

    pub fn pre_tick(&self, globals: &Globals) {
        for (id, fnpc) in self.table.iter() {
            let mut npc = fnpc.borrow_mut();
            if let Some(loc) = self.location_of.fwd().get(id) {
                let viewport = Viewport { 
                    rect: rect(-npc.view_radius, -npc.view_radius, 2 * npc.view_radius + 1, 2 * npc.view_radius + 1),
                    observer_in_rect: point2(0, 0),
                    observer: loc.facing(npc.facing),
                };
                globals.terrain.recalculate_egosphere(&mut npc.egosphere, viewport, |x| globals.at(x.point()).is_blocked())
            }
        }
    }

    pub fn tick(&self, globals: &Globals) {
        let location_of = &self.location_of;
        for (id, fnpc) in self.table.iter() {
            let mut npc = fnpc.borrow_mut();
            if let Some(loc) = location_of.fwd().get(id) {
                let mut tmp_move_ai = MoveAI::Idle;
                let facing = Cell::new(npc.facing);
                mem::swap(&mut npc.move_ai, &mut tmp_move_ai);

                let g = globals.clone();
                tmp_move_ai.advance(
                    |xy| {
                        if let Some(v) = npc.egosphere.at(xy) {
                            return g.at(v.point()).is_blocked();
                        }
                        return true;
                    },
                    |amt| {
                        location_of.fwd().insert(id, g.terrain.step_offset(loc.facing(facing.get()), amt).point());
                    },
                );

                mem::swap(&mut npc.move_ai, &mut tmp_move_ai);
                npc.facing = facing.get();
            }
        }
    }
}

pub struct NPC {
    pub facing: Cardinal,
    pub move_ai: MoveAI,
    pub view_radius: isize,
    pub egosphere: Egosphere,
}

pub enum MoveAI {
    Idle,
    Hotline(Hotline)
}

// hotline miami-style "hug the right wall" AI

impl MoveAI {
    pub fn advance(&mut self, blocked: impl Fn(EgoPoint) -> bool, step: impl FnOnce(EgoVec)) {
        match self {
            MoveAI::Idle => {}
            MoveAI::Hotline(h) => h.advance(blocked, step)
        }
    }
}