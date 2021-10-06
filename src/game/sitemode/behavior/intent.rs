use crate::game::reexports::*;

#[derive(Debug)]
pub struct Intent {
    // TODO: High prio stuff
    // next priority down
    pub walk: Movement,

    horiz_cooldown: usize,
    vert_cooldown: usize,
}

impl Intent {
    pub(crate) fn new() -> Intent {
        Intent { 
            walk: Movement { up: false, left: false, right: false, down: false }, 
            horiz_cooldown: 0,
            vert_cooldown: 0,
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
    pub(in crate::game::sitemode) fn handle_intent(&mut self, globals: &Rc<GlobalState>) {
        self.take_actions(globals);
        self.discharge_cooldowns();
    }

    fn take_actions(&mut self, globals: &Rc<GlobalState>) {
        // TODO: First prio

        // next prio: move
        let mut move_vec = self.intent.walk.vector();
        if self.intent.horiz_cooldown > 0 { move_vec.x = 0; }
        if self.intent.vert_cooldown > 0 { move_vec.y = 0; }
        let move_vec = self.walk(globals, move_vec);
        if move_vec != EgoVec::zero() {
            if move_vec.x != 0 {
                self.intent.horiz_cooldown = if self.intent.vert_cooldown == 0 { 4 } else { self.intent.vert_cooldown };
            }
            if move_vec.y != 0 {
                self.intent.vert_cooldown = if self.intent.horiz_cooldown == 0 { 4 } else { self.intent.horiz_cooldown };
            }
        }
    }
    
    fn discharge_cooldowns(&mut self) {
        // first prio: discharge cooldown
        if self.intent.horiz_cooldown > 0 { self.intent.horiz_cooldown -= 1; }
        if self.intent.vert_cooldown > 0 { self.intent.vert_cooldown -= 1; }
    }

    fn walk(&mut self, globals: &Globals, offset: EgoVec) -> EgoVec {
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