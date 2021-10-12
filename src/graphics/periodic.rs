use crate::reexports::*;

impl Graphics {
    pub fn pre_move_post_move_or_resize(&mut self, globals: &Globals, screen_boundaries: CellRect) {
        let player = globals.player.borrow();

        let new_xy = player.cumulative_xy_shift;
        if let Some(old_xy) = self.old_xy {
            self.memory.shift(-(new_xy - old_xy));
        }
        self.old_xy = Some(new_xy);

        self.calculate_viewport(screen_boundaries, &player);
        if let Some(viewport) = self.viewport {
            globals.terrain.recalculate_egosphere(&mut self.egosphere, viewport, |x| globals.at(x.point()).is_blocked());
            let ego = &self.egosphere;

            self.memory.resize(viewport); // TODO: 3x larger
            self.memory.calculate(|xy| Self::vis_cell(&globals, ego.at(xy)))
        }
    }

    pub fn post_tick_or_resize(&mut self, _globals: &Globals, _screen_boundaries: CellRect) {
        // TODO: Anything? Probably not. Maybe store the player's last position for shifting reasons
    }
}