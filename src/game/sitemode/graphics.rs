use crate::game::reexports::*;

const FADE: (u8, u8) = (colors::Dark[1], colors::Light[3]);
const TOP: (u8, u8) = (colors::Light[3], colors::Light[1]);

pub(super) const SCCELL_X: isize = 6;
pub(super) const SCCELL_Y: isize = 6;

impl SiteMode {
    pub fn draw<'frame>(&self, globals: &Globals, brush: Brush, menu: WidgetMenu<'frame, CanvasState>) {
        self.add_basic_controls(globals, menu);
        brush.fill(FSem::new().color(FADE));

        if let Some(viewport) = self.get_viewport(brush.rect()) {
            for ego_xy in isize::points_in(viewport.rect) {
                let screen_xy: CellPoint = point2(ego_xy.x * SCCELL_X, ego_xy.y * SCCELL_Y);
                let ego_xy_behind = ego_xy - vec2(0, 1);

                let viscell_behind = self.vis_cell(globals, self.egosphere.at(ego_xy_behind));
                let viscell_here = self.vis_cell(globals, self.egosphere.at(ego_xy));

                // TODO: Memory

                if let Some(behind) = viscell_behind {
                    if ego_xy_behind.y >= viewport.observer_in_rect.y {
                        // TODO: Remembered = true
                    }

                    behind.draw_front(brush.region(Rect::new(
                        screen_xy - vec2(0, 8),
                        size2(8, 8),
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

    fn vis_cell(&self, globals: &GlobalState, at: Option<GlobalView>) -> Option<VisCell> {
        // NOTE: if we don't know what's in it we can't use it
        let block = if let Some(x) = at {
            globals.terrain.borrow_mut().get(x.point())
        } else {
            return None
        };

        Some(match block {
            Block::Plain => VisCell { 
                height: 2,
                msg: format!("{:?},{:?}\n{:?}\n{:?}", at.unwrap().x.x, at.unwrap().x.y, at.unwrap().c, at.unwrap().r.get_value()),
            },
            Block::Empty => VisCell { 
                height: 0,
                msg: format!("{:?},{:?}\n{:?}\n{:?}", at.unwrap().x.x, at.unwrap().x.y, at.unwrap().c, at.unwrap().r.get_value()),
            },
        })
    }

    fn draw_player(&self, brush: Brush) {
        brush.region(rect(SCCELL_X / 2 - 1, SCCELL_Y / 2 - 1, 2, 2)).font(Font::Set).color((colors::Dark[0], colors::LtGreen[2])).putch(b'@');
    }
}

struct VisCell {
    height: usize,
    msg: String,
}

impl VisCell {
    fn draw_front(&self, brush: Brush) {
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

    fn draw_top(&self, brush: Brush) {
        brush.font(Font::Small).putfs(&self.msg);
        if self.height > 0 {
            // TODO: FADE if remembered
            brush.fill(FSem::new().color(TOP))
        }
    }
}