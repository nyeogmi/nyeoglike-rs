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

    pub fn input_press_activate(&mut self) {
        // Figure out what contextual action the player wants
        self.try_queue(ActivateToken);
        /*
        g.graphics.borrow_mut().show_contextual(Contextual::Side(AnyWidget::wrap(Label::new().setup(
            |l| l.set_text("NYEH")
        ))));
        */
    }
}