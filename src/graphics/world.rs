use crate::reexports::*;
use super::constants::EMPTY_FADE;

impl Graphics {
    pub fn draw_world<'frame>(&self, globals: &Globals, brush: Brush, menu: WidgetMenu<'frame, CanvasState>) {
        let player = globals.player.borrow();
        player.add_basic_controls(globals, menu);
        brush.fill(FSem::new().color(EMPTY_FADE));

        if let Some(viewport) = self.viewport {
            for ego_xy in isize::points_in(viewport.rect) {
                let screen_xy: CellPoint = point2(ego_xy.x * SCCELL_X, ego_xy.y * SCCELL_Y);
                let ego_xy_behind = ego_xy - vec2(0, 1);

                let mut viscell_behind = Self::vis_cell(globals, self.egosphere.at(ego_xy_behind));
                let mut viscell_here = Self::vis_cell(globals, self.egosphere.at(ego_xy));

                if let None = viscell_behind {
                    viscell_behind = self.memory.remember(ego_xy_behind);
                    viscell_behind = viscell_behind.map(|mut vc| { vc.degrade_memory(); vc });
                }
                if let None = viscell_here {
                    viscell_here = self.memory.remember(ego_xy);
                    viscell_here = viscell_here.map(|mut vc| { vc.degrade_memory(); vc });
                }

                if let Some(here) = viscell_here {
                    here.draw_base(brush.region(Rect::new(screen_xy, size2(SCCELL_X, SCCELL_Y))));
                }

                if let Some(mut behind) = viscell_behind {
                    if ego_xy_behind.y >= viewport.observer_in_rect.y {
                        behind.degrade_memory();
                    }

                    behind.draw_front(brush.region(Rect::new(
                        screen_xy,
                        size2(SCCELL_X, SCCELL_Y),
                    )));
                }

                if let Some(here) = viscell_here {
                    here.draw_contents(brush.region(Rect::new(screen_xy, size2(SCCELL_X, SCCELL_Y))));
                    here.draw_top(brush.region(Rect::new(screen_xy, size2(SCCELL_X, SCCELL_Y))));

                    // TODO: Check if remembered and if not, draw cell content
                }
            }
        }
    }

    pub fn vis_cell(globals: &GlobalState, at: Option<GlobalView>) -> Option<VisCell> {
        // NOTE: if we don't know what's in it we can't use it
        let cursor = if let Some(x) = at {
            globals.at(x.point())
        } else {
            return None
        };

        let mut content: SmallVec<[VisContent; 4]> = SmallVec::new();

        if Some(cursor.point()) == globals.player.borrow().xy.map(|x| x.point()) {
            content.push(VisContent { fg: colors::LtGreen[2], char: b'@' as u16 });
        }

        for _ in cursor.npcs().iter().rev() {
            content.push(VisContent { fg: colors::LtYellow[2], char: b'@' as u16 });
        }

        for x in cursor.items().iter().rev() {
            if let Some(spawn) = globals.items.spawns.get(x) {
                let icon = spawn.borrow().item.profile.icon;
                content.push(VisContent { fg: icon.fg, char: icon.art })
            }
        }

        let mut iter = content.iter().cloned();
        let content = [iter.next(), iter.next(), iter.next(), iter.next()];

        Some(match cursor.get_block() {
            Block::Plain => VisCell { 
                filled: true,
                remembered: false,
                content,
            },
            Block::Empty => {
                VisCell { 
                    filled: false,
                    remembered: false,
                    content,
                }
            }
        })
    }
}