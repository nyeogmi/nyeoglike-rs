use crate::game::reexports::*;

impl SiteMode {
    pub fn input_set_walk_up(&mut self, is_down: bool) {
        self.behavior.walk.up = is_down;
    }
    pub fn input_set_walk_down(&mut self, is_down: bool) {
        self.behavior.walk.down = is_down;
    }
    pub fn input_set_walk_left(&mut self, is_down: bool) {
        self.behavior.walk.left = is_down;
    }
    pub fn input_set_walk_right(&mut self, is_down: bool) {
        self.behavior.walk.right = is_down;
    }
}