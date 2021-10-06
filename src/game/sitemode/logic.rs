use crate::game::reexports::*;

use super::{Intent, graphics::{SCCELL_X, SCCELL_Y}};

impl SiteMode {
    pub fn on_loop(&mut self, globals: &Globals, screen_boundaries: CellRect) {
        // place player if possible
        if let None = self.player_xy {
            self.player_xy = globals.terrain.borrow().suggest_player_xy();
        }

        // update viewport
        if let Some(viewport) = self.get_viewport(screen_boundaries) {
            globals.terrain.borrow().recalculate_egosphere(&mut self.egosphere, viewport)
        }
    }

    pub(super) fn get_viewport(&self, screen_boundaries: CellRect) -> Option<Viewport> {
        let ego_rect = rect(
            0, 0, 
            // TODO: Round up
            screen_boundaries.width() / SCCELL_X + 1, screen_boundaries.height() / SCCELL_Y + 1
        );

        if let Some(player_xy) = self.player_xy {
            Some(Viewport {
                rect: ego_rect,
                observer_in_rect: ego_rect.center().cast_unit(),
                observer: player_xy,
            })
        } else {
            None
        }
    }

    pub fn on_tick(&mut self, globals: &Globals) {
        self.handle_intent(globals);
    }
}