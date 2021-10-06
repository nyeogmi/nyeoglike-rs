use crate::game::reexports::*;

use super::{WalkToken, walk::Walk};

#[derive(Debug)]
pub struct Behavior {
    // TODO: High prio stuff
    // next priority down
    pub walk: Walk,
}

impl Behavior {
    pub(crate) fn new() -> Behavior {
        Behavior { 
            walk: Walk::new(),
        }
    }
}

impl SiteMode {
    pub(in crate::game::sitemode) fn handle_intent(&mut self, globals: &Globals) {
        self.take_actions(globals);
        self.discharge_cooldowns(globals);
    }

    fn take_actions(&mut self, globals: &Globals) {
        // next prio: move
        if self.act(globals, WalkToken) { return }
    }
    
    fn discharge_cooldowns(&mut self, globals: &Globals) {
        self.discharge_cooldown(globals, WalkToken)
    }

}