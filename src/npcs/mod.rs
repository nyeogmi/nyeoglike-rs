mod hotline;

use crate::reexports::*;
pub use hotline::Hotline;

pub struct NPCs {
    pub table: Pom<NPC>,

    // TODO: Don't let people use this directly (so we can force an egosphere recalc)
    pub location_of: OneToOne<Id<NPC>, GlobalPoint>,
}

impl NPCs {
    pub fn new() -> NPCs {
        NPCs {  
            table: Pom::new(),
            location_of: OneToOne::new(),
        }
    }

    pub fn create_npc(&mut self, facing: Cardinal, move_ai: MoveAI, view_radius: isize) -> Id<NPC> {
        self.table.insert(NPC {
            facing,
            move_ai,
            view_radius,
            egosphere: Egosphere::new(false),
        })
    }

    pub fn pre_tick(&mut self, globals: &Globals) {
        let terrain = globals.terrain.borrow_mut();
        for (id, npc) in self.table.iter_mut() {
            if let Some(loc) = self.location_of.fwd().get(id) {
                let viewport = Viewport { 
                    rect: rect(-npc.view_radius, -npc.view_radius, 2 * npc.view_radius + 1, 2 * npc.view_radius + 1),
                    observer_in_rect: point2(0, 0),
                    observer: loc.facing(npc.facing),
                };
                terrain.recalculate_egosphere(&mut npc.egosphere, viewport)
            }
        }
    }

    pub fn tick(&mut self, globals: &Globals) {
        let location_of = &mut self.location_of;
        for (id, npc) in self.table.iter_mut() {
            if let Some(loc) = location_of.fwd().get(id) {
                let mut tmp_move_ai = MoveAI::Idle;
                let facing = Cell::new(npc.facing);
                mem::swap(&mut npc.move_ai, &mut tmp_move_ai);

                let terr = globals.terrain.borrow_mut();
                tmp_move_ai.advance(
                    |xy| {
                        if let Some(g) = npc.egosphere.at(xy) {
                            return terr.get(g.point()).is_blocked();
                        }
                        return true;
                    },
                    |ego| {
                        facing.replace(facing.get().rotated(ego));
                    },
                    |amt| {
                        location_of.fwd().insert(id, terr.step_offset(loc.facing(facing.get()), amt).point());
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
    pub fn advance(&mut self, blocked: impl Fn(EgoPoint) -> bool, turn: impl FnMut(Egocentric), step: impl FnOnce(EgoVec)) {
        match self {
            MoveAI::Idle => {}
            MoveAI::Hotline(h) => h.advance(blocked, turn, step)
        }
    }
}