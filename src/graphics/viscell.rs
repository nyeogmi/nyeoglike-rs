use crate::reexports::*;

use super::constants::*;

#[derive(Clone, Copy, Debug)]
pub struct VisCell {
    pub height: usize,
    pub remembered: bool,
    // msg: String,
}

impl VisCell {
    pub fn draw_front(&self, brush: Brush) {
        if self.height > 0 {
            // TODO: FADE if remembered
            if self.remembered {
                brush.fill(FSem::new().color(SIDE_FADE))
            } else {
                brush.fill(FSem::new().color(SIDE))
            }
        }
    }

    pub fn draw_base(&self, brush: Brush) {
        if self.remembered {
        } else {
            brush.fill(FSem::new().color(EMPTY))
        }
    }

    pub fn draw_top(&self, brush: Brush) {
        if self.height > 0 {
            if self.remembered {
                brush.fill(FSem::new().color(TOP_FADE))
            }
            else {
                brush.fill(FSem::new().color(TOP))
            }
        } 
    }
}