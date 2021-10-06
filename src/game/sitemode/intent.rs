use crate::game::reexports::*;

#[derive(Debug)]
pub struct Intent {
    // TODO: High prio stuff
    // next priority down
    pub movement: Movement,

    cooldown: usize,
}

impl Intent {
    pub(crate) fn new() -> Intent {
        Intent { 
            movement: Movement { up: false, left: false, right: false, down: false }, 
            cooldown: 0 
        }
    }
}

#[derive(Debug)]
pub struct Movement {
    pub up: bool, 
    pub left: bool, 
    pub right: bool, 
    pub down: bool,
}

impl SiteMode {
    pub(super) fn handle_intent(&mut self, globals: &Rc<GlobalState>) {
        if self.intent.cooldown > 0 {
            self.intent.cooldown -= 1;
        }
        else {
            let move_vec = self.intent.movement.vector();
            if move_vec != EgoVec::zero() {
                self.walk(globals, move_vec);
                self.intent.cooldown = 4;
            }
        }
    }

    fn walk(&mut self, globals: &Globals, offset: EgoVec) {
        let terrain: Ref<Terrain> = globals.terrain.borrow();
        self.player_xy = self.player_xy.map(|pxy| terrain.step_offset(pxy, offset));
    }
}
impl Movement {
    pub(crate) fn vector(&self) -> EgoVec {
        vec2(
            if self.left { -1 } else { 0 } +
            if self.right { 1 } else { 0 },
            
            if self.up { -1 } else { 0 } +
            if self.down { 1 } else { 0 },
        )
    }
}