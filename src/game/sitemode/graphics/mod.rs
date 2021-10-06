pub(self) mod constants;
mod memory;
mod viscell;
mod visibility;

pub use memory::Memory;

use self::constants::FADE;
pub(in crate::game::sitemode) use self::constants::{SCCELL_X, SCCELL_Y};
pub use self::viscell::VisCell;

use crate::game::reexports::*;

impl SiteMode {
    pub fn draw<'frame>(&self, globals: &Globals, brush: Brush, menu: WidgetMenu<'frame, CanvasState>) {
        self.add_basic_controls(globals, menu);
        brush.fill(FSem::new().color(FADE));

        if let Some(viewport) = self.get_viewport(brush.rect()) {
            for ego_xy in isize::points_in(viewport.rect) {
                let screen_xy: CellPoint = point2(ego_xy.x * SCCELL_X, ego_xy.y * SCCELL_Y);
                let ego_xy_behind = ego_xy - vec2(0, 1);

                let mut viscell_behind = Self::vis_cell(globals, self.egosphere.at(ego_xy_behind));
                let mut viscell_here = Self::vis_cell(globals, self.egosphere.at(ego_xy));

                if let None = viscell_behind {
                    viscell_behind = self.memory.remember(ego_xy_behind);
                    viscell_behind = viscell_behind.map(|mut vc| { vc.remembered = true; vc });
                }
                if let None = viscell_here {
                    viscell_here = self.memory.remember(ego_xy);
                    viscell_here = viscell_here.map(|mut vc| { vc.remembered = true; vc });
                }

                // TODO: Memory

                if let Some(mut behind) = viscell_behind {
                    if ego_xy_behind.y >= viewport.observer_in_rect.y {
                        behind.remembered = true;
                    }

                    behind.draw_front(brush.region(Rect::new(
                        screen_xy,
                        size2(SCCELL_X, SCCELL_Y),
                    )));
                }

                if let Some(here) = viscell_here {
                    here.draw_top(brush.region(Rect::new(screen_xy, size2(SCCELL_X, SCCELL_Y))));

                    // TODO: Check if remembered and if not, draw cell content
                }

                if ego_xy == viewport.observer_in_rect {
                    self.draw_player(brush.region(Rect::new(screen_xy, size2(SCCELL_X, SCCELL_Y))))
                }
            }
        }
    }

    fn vis_cell(globals: &GlobalState, at: Option<GlobalView>) -> Option<VisCell> {
        // NOTE: if we don't know what's in it we can't use it
        let block = if let Some(x) = at {
            globals.terrain.borrow_mut().get(x.point())
        } else {
            return None
        };

        Some(match block {
            Block::Plain => VisCell { 
                height: 2,
                remembered: false,
                // msg: format!("{:?},{:?}\n{:?}\n{:?}", at.unwrap().x.x, at.unwrap().x.y, at.unwrap().c, at.unwrap().r.get_value()),
            },
            Block::Empty => VisCell { 
                height: 0,
                remembered: false,
                // msg: format!("{:?},{:?}\n{:?}\n{:?}", at.unwrap().x.x, at.unwrap().x.y, at.unwrap().c, at.unwrap().r.get_value()),
            },
        })
    }

    fn draw_player(&self, brush: Brush) {
        brush.region(rect(SCCELL_X / 2 - 1, SCCELL_Y / 2 - 1, 2, 2)).font(Font::Set).color((colors::Dark[0], colors::LtGreen[2])).putch(b'@');
    }
}
