use crate::{reexports::*, player::*};

pub trait CanPerform<T: Clone> {
    // these can do nothing if your type has no queuing feature
    fn get_activity_state(&self, _token: T) -> ActivityState { ActivityState::Exempt }
    fn internal_mark_queuing(&mut self, _token: T, _queuing: bool) { }

    // this will definitely don othing if your type has no queuing feature
    fn handle_auxiliary(&mut self, _token: T, _auxiliary: Auxiliary) -> bool { false }

    // do the thing
    fn act(&mut self, globals: &Globals, token: T) -> bool;
    fn cooldown(&mut self, globals: &Globals, token: T);
}