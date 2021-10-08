use crate::foreach_behavior;
use crate::player::*;

impl Player {
    // TODO: Track last auxiliary pressed and use it
    pub fn input_auxiliary(&mut self, auxiliary: Auxiliary) {
        foreach_behavior! { [behavior]
            if self.handle_auxiliary(behavior, auxiliary) {
                return;
            }
        }
    }

    pub fn input_press_charge(&mut self) {
        self.try_queue(ChargeToken);
    }
}