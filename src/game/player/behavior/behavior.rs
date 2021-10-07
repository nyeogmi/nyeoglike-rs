use crate::game::{reexports::*};

use super::*;

#[derive(Debug)]
pub struct Behavior {
    // high priority: modal actions
    pub charge: Charge,

    // next priority down
    pub walk: Walk,
}

impl Behavior {
    pub(crate) fn new() -> Behavior {
        Behavior { 
            charge: Charge::new(),
            walk: Walk::new(),
        }
    }
}

#[macro_export]
macro_rules! foreach_behavior {
    ([$x: ident] $($body: tt)*) => {
        let $x = ChargeToken; $($body)*
        let $x = WalkToken; $($body)*
    };
}

impl Player {
    pub(in crate::game::player) fn handle_intent(&mut self, globals: &Globals) {
        self.take_actions(globals);
        self.discharge_cooldowns(globals);
    }

    fn take_actions(&mut self, globals: &Globals) {
        foreach_behavior! { [behavior] 
            if self.act(globals, behavior) { return }
        }
    }
    
    fn discharge_cooldowns(&mut self, globals: &Globals) {
        foreach_behavior! { [behavior] 
            self.cooldown(globals, behavior);
        }
    }

    pub fn try_queue<T: Clone>(&mut self, tok: T) where Self: CanPerform<T> {
        if !self.ready(tok.clone()) { return }
        if self.busy() { return }

        self.dequeue_all();
        self.internal_mark_queuing(tok.clone(), true);
    }

    fn ready<T: Clone>(&self, tok: T) -> bool where Self: CanPerform<T> {
        match self.get_activity_state(tok) {
            ActivityState::Ready => true,
            _ => false
        }
    }

    fn busy(&self) -> bool {
        foreach_behavior! { [behavior] 
            if self.get_activity_state(behavior) == ActivityState::Busy {
                return true
            }
        }
        return false;
    }

    fn dequeue_all(&mut self) {
        foreach_behavior! { [behavior] 
            if self.get_activity_state(behavior) == ActivityState::Queuing {
                self.internal_mark_queuing(behavior, false);
            }
        }
    }
}