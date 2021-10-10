use crate::reexports::*;

impl Player {
    pub fn on_tick(&mut self, globals: &Globals) {
        if let None = self.xy {
            self.xy = globals.terrain.suggest_player_xy();
        }

        self.handle_intent(globals);
    }
}