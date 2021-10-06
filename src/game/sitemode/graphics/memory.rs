use crate::game::reexports::*;

use areaportal2d::{EgoVec};

use super::VisCell;

pub struct Memory {
    grid: Grid<Option<VisCell>, EgoSpace>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            grid: Grid::new(rect(0, 0, 0, 0), || unreachable!()),
        }
    }

    pub fn resize(&mut self, viewport: Viewport) {
        let original_rect = self.grid.rect();
        if original_rect == viewport.rect { return; }

        let new_rect = viewport.rect;
        self.grid.resize(new_rect, || None);
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