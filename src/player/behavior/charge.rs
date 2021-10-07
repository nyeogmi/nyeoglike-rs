use std::collections::VecDeque;

use crate::reexports::*;
use crate::player::*;

#[derive(Debug)]
pub struct Charge {
    queuing: bool,
    forced_actions: VecDeque<EgoVec>,
    cooldown: usize,
}

#[derive(Clone, Copy)]
pub struct ChargeToken;

impl Charge {
    pub(in crate::player) fn new() -> Charge {
        Charge {  
            queuing: false,
            forced_actions: VecDeque::new(),
            cooldown: 0,
        }
    }
}

impl CanPerform<ChargeToken> for Player {
    fn get_activity_state(&self, _token: ChargeToken) -> ActivityState {
        if self.behavior.charge.queuing { return ActivityState::Queuing; }
        if self.behavior.charge.forced_actions.len() > 0 { return ActivityState::Busy; }
        if self.behavior.charge.cooldown > 0 { return ActivityState::Cooldown; }
        return ActivityState::Ready;
    }

    fn internal_mark_queuing(&mut self, _token: ChargeToken, queuing: bool) { 
        self.behavior.charge.queuing = queuing
    }

    fn handle_auxiliary(&mut self, _token: ChargeToken, auxiliary: Auxiliary) -> bool { 
        if !self.behavior.charge.queuing { return false }

        use Auxiliary::*;
        // TODO: Hold to wind up, release to uh, release
        let active_offset = match auxiliary {
            Up(true) => vec2(0, -1),
            Down(true) => vec2(0, 1),
            Left(true) => vec2(-1, 0),
            Right(true) => vec2(1, 0),
            _ => { return false }  
        };

        self.behavior.charge.cooldown = 2;
        self.behavior.charge.forced_actions.push_back(active_offset * 6);
        self.behavior.charge.forced_actions.push_back(active_offset * 4);
        self.behavior.charge.forced_actions.push_back(active_offset * 4);
        self.behavior.charge.forced_actions.push_back(active_offset * 2);
        self.behavior.charge.forced_actions.push_back(active_offset);
        self.behavior.charge.queuing = false;

        return true;
    }

    fn act(&mut self, globals: &Globals, _token: ChargeToken) -> bool {
        // TODO
        if let Some(a) = self.behavior.charge.forced_actions.pop_front() {
            self.move_by(globals, a, true);
            return true
        } else if self.behavior.charge.queuing == true {
            return true  // preclude other options
        } {
            return false
        }
    }

    fn cooldown(&mut self, _globals: &Globals, _token: ChargeToken) {
        if self.behavior.charge.forced_actions.len() > 0 {
            // don't cool down at all!
        };
        if self.behavior.charge.cooldown > 0 {
            self.behavior.charge.cooldown -= 1;
        }
    }
}