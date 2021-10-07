use crate::game::reexports::*;
use crate::game::player::*;

#[derive(Debug)]
pub struct Walk {
    // intent vars (public)
    pub up: bool, 
    pub left: bool, 
    pub right: bool, 
    pub down: bool,

    // cooldown vars, constraining intent
    horiz_cooldown: usize,
    vert_cooldown: usize,
}

#[derive(Clone, Copy)]
pub struct WalkToken;

impl Walk {
    pub(in crate::game::player) fn new() -> Walk {
        Walk {
            up: false, left: false, right: false, down: false,
            horiz_cooldown: 0, vert_cooldown: 0,
        }
    }

    pub(crate) fn vector(&self) -> EgoVec {
        vec2(
            if self.left { -1 } else { 0 } +
            if self.right { 1 } else { 0 },
            
            if self.up { -1 } else { 0 } +
            if self.down { 1 } else { 0 },
        )
    }
}

impl CanPerform<WalkToken> for Player {
    fn handle_auxiliary(&mut self, _token: WalkToken, auxiliary: Auxiliary) -> bool {
        match auxiliary {
            Auxiliary::Up(u) => {
                self.behavior.walk.up = u;
                return true
            }
                
            Auxiliary::Down(d) => {
                self.behavior.walk.down = d;
                return true
            }
            Auxiliary::Left(l) => {
                self.behavior.walk.left = l;
                return true
            }
            Auxiliary::Right(r) => {
                self.behavior.walk.right = r;
                return true
            }
        }
    }

    fn act(&mut self, globals: &Globals, _token: WalkToken) -> bool {
        let mut move_vec = self.behavior.walk.vector();
        if self.behavior.walk.horiz_cooldown > 0 { move_vec.x = 0; }
        if self.behavior.walk.vert_cooldown > 0 { move_vec.y = 0; }
        let move_vec = self.move_by(globals, move_vec, false);
        if move_vec == EgoVec::zero() {
            return false
        } else {
            if move_vec.x != 0 {
                self.behavior.walk.horiz_cooldown = if self.behavior.walk.vert_cooldown == 0 { 
                    3 
                } else { 
                    self.behavior.walk.vert_cooldown 
                };
            }
            if move_vec.y != 0 {
                self.behavior.walk.vert_cooldown = if self.behavior.walk.horiz_cooldown == 0 { 
                    3 
                } else { 
                    self.behavior.walk.horiz_cooldown 
                };
            }
            return true
        }
    }

    fn cooldown(&mut self, _globals: &Globals, _token: WalkToken) {
        if self.behavior.walk.horiz_cooldown > 0 { self.behavior.walk.horiz_cooldown -= 1; }
        if self.behavior.walk.vert_cooldown > 0 { self.behavior.walk.vert_cooldown -= 1; }
    }
}