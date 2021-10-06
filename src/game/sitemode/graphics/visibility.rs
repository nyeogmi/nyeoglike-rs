use crate::game::reexports::*;

use super::{SCCELL_X, SCCELL_Y};

impl SiteMode {
    pub(in crate::game::sitemode) fn shift_memory(&mut self, offset: EgoVec) {
        self.memory.shift(offset)
    }

    pub(in crate::game::sitemode) fn update_visibility(&mut self, globals: &Globals, screen_boundaries: CellRect) {
        // update viewport
        if let Some(viewport) = self.get_viewport(screen_boundaries) {
            globals.terrain.borrow().recalculate_egosphere(&mut self.egosphere, viewport);
            let ego = &self.egosphere;

            self.memory.resize(viewport); // TODO: 3x larger

            // NYEO TODO: recalculate memory if:
            // - the viewport changes size
            // - the player moves
            // - something in the world moves
            // only recalculate memory on ticks
            // probably provide a mark_dirty function that resize() calls!
            // NOTE: Maybe a function that is called once every tick, but instantly re-called if the screen is resized?
            self.memory.calculate(|xy| Self::vis_cell(&globals, ego.at(xy)))
        }
    }

    pub(in crate::game::sitemode) fn get_viewport(&self, screen_boundaries: CellRect) -> Option<Viewport> {
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
}