use crate::game::reexports::*;

use areaportal2d::{EgoVec};

use super::VisCell;

pub struct Memory {
    viewport: Option<Viewport>,
    grid: Grid<Option<VisCell>, EgoSpace>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            viewport: None,
            grid: Grid::new(rect(0, 0, 0, 0), || unreachable!()),
        }
    }

    pub fn resize(&mut self, viewport: Viewport) {
        let shift_amt = if let Some(old_viewport) = self.viewport {
            old_viewport.observer_in_rect - viewport.observer_in_rect
        } else {
            vec2(0, 0)
        };

        let original_rect = self.grid.rect();
        if original_rect == viewport.rect { return; }

        let new_rect = viewport.rect;
        self.grid.resize(new_rect, || None);

        if shift_amt != vec2(0, 0) {
            self.shift(-shift_amt)
        }
        self.viewport = Some(viewport);
    }

    pub fn shift(&mut self, offset: EgoVec) {
        let mut grid2 = Grid::new(self.grid.rect(), || None);
        for xy in isize::points_in(self.grid.rect()) {
            if let Some(val) = self.grid.get(xy - offset) {
                grid2.set(xy, *val);
            }
        }
        self.grid = grid2;
    }

    pub fn calculate(&mut self, look: impl Fn(EgoPoint) -> Option<VisCell>) {
        for xy in isize::points_in(self.grid.rect()) {
            let vis = look(xy);
            if let Some(vis) = vis {
                self.grid.set(xy, Some(vis))
            }
        }
    }

    pub fn remember(&self, xy: EgoPoint) -> Option<VisCell> {
        match self.grid.get(xy) {
            Some(Some(x)) => {
                Some(*x)
            },
            _ => None,
        }
    }
}