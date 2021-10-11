use crate::reexports::*;

use super::constants::*;

#[derive(Clone, Copy, Debug)]
pub struct VisCell {
    pub filled: bool,
    pub remembered: bool,
    // TODO: Allow plural of these
    pub content: [Option<VisContent>; 4],
    // msg: String,
}

// TODO: Centralize this hack somewhere
#[derive(Clone, Copy, Debug)]
pub struct VisContent { pub fg: u8, pub char: u16 }

impl VisCell {
    pub fn degrade_memory(&mut self) {
        self.remembered = true;
        // TODO: Only remove NPCs
        self.content = [None, None, None, None];  // don't show the user npcs in tiles that are merely remembered
    }

    pub fn draw_front(&self, brush: Brush) {
        if self.filled {
            // TODO: FADE if remembered
            if self.remembered {
                brush.fill(FSem::new().sem(SemanticContent::Small(0xb0)).color(SIDE_FADE))
            } else {
                brush.fill(FSem::new().sem(SemanticContent::Small(0xb0)).color(SIDE))
            }
        }
    }

    pub fn draw_base(&self, brush: Brush) {
        if self.remembered {
        } else {
            brush.fill(FSem::new().sem(SemanticContent::Blank).color(EMPTY))
        }
    }

    pub fn draw_contents(&self, brush: Brush) {
        if self.remembered { return }  // TODO: The degrade memory code should handle this case
        
        let mut stuff: SmallVec<[VisContent; 4]> = SmallVec::new();
        for i in self.content {
            if let Some(x) = i { stuff.push(x) }
        }

        let stuff_zone = brush.region(rect(SCCELL_X / 2 - 1, SCCELL_Y / 2 - 1, 2, 2));
        match stuff.len() {
            0 => { }
            1 => { 
                let x = stuff[0];
                stuff_zone.font(Font::Set).fg(x.fg).putch(x.char);
            } 
            2 => {
                let x = stuff[0];
                let y = stuff[1];
                stuff_zone.font(Font::Normal).at(point2(0, 0)).fg(x.fg).putch(x.char);
                stuff_zone.font(Font::Normal).at(point2(1, 0)).fg(y.fg).putch(y.char);
            }
            3 => {
                let x = stuff[0];
                let y = stuff[1];
                let z = stuff[2];
                stuff_zone.at(point2(0, 0)).font(Font::Normal).fg(x.fg).putch(x.char);
                stuff_zone.at(point2(1, 0)).font(Font::Small).fg(y.fg).putch(y.char);
                stuff_zone.at(point2(1, 1)).font(Font::Small).fg(z.fg).putch(z.char);
            }
            _ => {
                let x = stuff[0];
                let y = stuff[1];
                let z = stuff[2];
                let w = stuff[3];
                stuff_zone.at(point2(0, 0)).font(Font::Small).fg(x.fg).putch(x.char);
                stuff_zone.at(point2(0, 1)).font(Font::Small).fg(y.fg).putch(y.char);
                stuff_zone.at(point2(1, 0)).font(Font::Small).fg(z.fg).putch(z.char);
                stuff_zone.at(point2(1, 1)).font(Font::Small).fg(w.fg).putch(w.char);
            }
        }
    }

    pub fn draw_top(&self, brush: Brush) {
        if self.filled {
            if self.remembered {
                brush.fill(FSem::new().sem(SemanticContent::Small(b'\xdb' as u16)).color(TOP_FADE))
            }
            else {
                brush.fill(FSem::new().sem(SemanticContent::Small(b'\xdb' as u16)).color(TOP))
            }
        } 
    }
}