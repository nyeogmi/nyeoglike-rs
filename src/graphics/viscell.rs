use crate::reexports::*;

use super::constants::*;

#[derive(Clone, Copy, Debug)]
pub struct VisCell {
    pub filled: bool,
    pub remembered: bool,
    pub npc: Option<Id<NPC>>, // note: in practice use the ID of the NPC
    // msg: String,
}

impl VisCell {
    pub fn degrade_memory(&mut self) {
        self.remembered = true;
        self.npc = None;  // don't show the user npcs in tiles that are merely remembered
    }

    pub fn draw_front(&self, brush: Brush) {
        if self.filled {
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

    pub fn draw_contents(&self, brush: Brush) {
        if self.remembered { return }  // TODO: The degrade memory code should handle this case
        
        // TODO: Centralize this hack somewhere
        if self.npc.is_some() {
            brush.region(rect(SCCELL_X / 2 - 1, SCCELL_Y / 2 - 1, 2, 2)).font(Font::Set).color((colors::Dark[0], colors::LtYellow[2])).putch(b'@');
        }
    }

    pub fn draw_top(&self, brush: Brush) {
        if self.filled {
            if self.remembered {
                brush.fill(FSem::new().color(TOP_FADE))
            }
            else {
                brush.fill(FSem::new().color(TOP))
            }
        } 
    }
}