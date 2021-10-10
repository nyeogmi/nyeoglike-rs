pub(self) mod constants;
mod memory;
mod viscell;

use self::constants::EMPTY_FADE;
pub(crate) use self::constants::{SCCELL_X, SCCELL_Y};
pub use self::memory::Memory;
pub use self::viscell::VisCell;

use crate::reexports::*;

pub struct Graphics {
    // viewport: Option<Viewport>,
    egosphere: Egosphere,
    memory: Memory,
    old_xy: Option<EgoVec>,
}

impl Graphics {
    pub fn new() -> Graphics {
        Graphics { 
            // viewport: None,
            egosphere: Egosphere::new(false),
            memory: Memory::new(),
            old_xy: None,
        }
    }
}

impl Graphics {
    pub fn pre_tick_or_resize(&mut self, globals: &Globals, screen_boundaries: CellRect) {
        let player = globals.player.borrow();

        let new_xy = player.cumulative_xy_shift;
        if let Some(old_xy) = self.old_xy {
            self.memory.shift(-(new_xy - old_xy));
        }
        self.old_xy = Some(new_xy);

        if let Some(viewport) = self.get_viewport(screen_boundaries, &player) {
            globals.terrain.recalculate_egosphere(&mut self.egosphere, viewport, |x| globals.at(x.point()).is_blocked());
            let ego = &self.egosphere;

            self.memory.resize(viewport); // TODO: 3x larger
            self.memory.calculate(|xy| Self::vis_cell(&globals, ego.at(xy)))
        }
    }

    pub fn post_tick_or_resize(&mut self, _globals: &Globals, _screen_boundaries: CellRect) {
        // TODO: Anything? Probably not. Maybe store the player's last position for shifting reasons
    }

    pub fn draw<'frame>(&self, globals: &Globals, brush: Brush, menu: WidgetMenu<'frame, CanvasState>) {
        let player = globals.player.borrow();
        player.add_basic_controls(globals, menu);
        brush.fill(FSem::new().color(EMPTY_FADE));

        if let Some(viewport) = self.get_viewport(brush.rect(), &player) {
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

                if ego_xy == viewport.observer_in_rect {
                    self.draw_player(brush.region(Rect::new(screen_xy, size2(SCCELL_X, SCCELL_Y))))
                }
            }
        }
    }

    fn vis_cell(globals: &GlobalState, at: Option<GlobalView>) -> Option<VisCell> {
        // NOTE: if we don't know what's in it we can't use it
        let (loc, block) = if let Some(x) = at {
            (x, globals.at(x.point()).get_block())
        } else {
            return None
        };

        Some(match block {
            Block::Plain => VisCell { 
                filled: true,
                remembered: false,
                npc: None,
            },
            Block::Empty => {
                let npc = globals.npcs.location_of.bwd().get(loc.point());
                VisCell { 
                    filled: false,
                    remembered: false,
                    npc,
                }
            }
        })
    }

    fn draw_player(&self, brush: Brush) {
        brush.region(rect(SCCELL_X / 2 - 1, SCCELL_Y / 2 - 1, 2, 2)).font(Font::Set).fg(colors::LtGreen[2]).putch(b'@');
    }

    fn get_viewport(&self, screen_boundaries: CellRect, player: &Player) -> Option<Viewport> {
        let ego_rect = rect(
            0, 0, 
            // TODO: Round up
            screen_boundaries.width() / SCCELL_X + 1, screen_boundaries.height() / SCCELL_Y + 1
        );

        if let Some(player_xy) = player.xy {
            Some(Viewport {
                rect: ego_rect,
                observer_in_rect: ego_rect.center().cast_unit(),
                observer: player_xy,
            })
        } else {
            None
        }
    }
}
