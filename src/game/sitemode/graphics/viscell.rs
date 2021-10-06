use crate::game::reexports::*;

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
            brush.fill(FSem::new().color(SIDE))
        }
        if self.height > 0 {
            /* 
            brush.at(point2(0, 0)).putch(0xb0u16);
            brush.at(point2(0, 2)).putch(0xb0u16);
            brush.at(point2(1, 0)).putch(0xb0u16);
            brush.at(point2(1, 2)).putch(0xb0u16);
            brush.at(point2(2, 0)).putch(0xb0u16);
            brush.at(point2(2, 2)).putch(0xb0u16);
            brush.at(point2(3, 0)).putch(0xb0u16);
            brush.at(point2(3, 2)).putch(0xb0u16);
            */
        }
    }

    pub fn draw_top(&self, brush: Brush) {
        // brush.font(Font::Small).putfs(&self.msg);
        if self.height > 0 {
            // TODO: FADE if remembered
            if self.remembered {
                brush.fill(FSem::new().color(TOP_FADE))
            }
            else {
                brush.fill(FSem::new().color(TOP))
            }
        }
    }
}