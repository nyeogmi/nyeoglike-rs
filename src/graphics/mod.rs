pub(self) mod constants;
mod contextual;
mod hud;
mod memory;
mod periodic;
mod theme;
mod viscell;
mod world;

pub(crate) use self::constants::{SCCELL_X, SCCELL_Y};
pub use self::contextual::*;
pub use self::memory::Memory;
pub use self::viscell::{VisCell, VisContent};
pub use self::theme::*;

use crate::reexports::*;

pub struct Graphics {
    viewport: Option<Viewport>,
    egosphere: Egosphere,
    memory: Memory,
    old_xy: Option<EgoVec>,
    pub mouse_xy: Option<CellPoint>,

    pub contextuals: ContextualServer,
}

impl Graphics {
    pub fn new() -> Graphics {
        Graphics { 
            viewport: None,
            egosphere: Egosphere::new(false),
            memory: Memory::new(),
            old_xy: None,
            mouse_xy: None,
            contextuals: ContextualServer::new(),
        }
    }
}

impl Graphics {
    pub fn main_loop(globals: &Globals, io: &mut IO) {
        let g = globals.clone();
        let sitemode_display = Canvas::new().setup(|c| {
            c.set_draw(move |brush, menu| {
                g.graphics.borrow_mut().draw_world(&g, brush, menu);
            });
        });

        let g = globals.clone();
        let hud_player_part = Canvas::new().setup(|c| {
            c.set_draw(move |brush, menu| {
                g.graphics.borrow_mut().draw_player_hud(&g, brush, menu)
            });
            c.layout_hacks.preferred_height = Some(6);
        });

        let hud_player = Window::new().setup(|w| {
            w.set_title("Nyeogmi");
            w.set(hud_player_part);
        });

        let g = globals.clone();
        let hud_time_part = Canvas::new().setup(|c| {
            c.set_draw(move |brush, menu| {
                g.graphics.borrow_mut().draw_time_hud(&g, brush, menu)
            });
            c.layout_hacks.preferred_height = Some(2);
            c.layout_hacks.preferred_width = Some(10);
        });

        let hud_time = Window::new().setup(|w| {
            w.set(hud_time_part);
        });

        let modal_container_side = Container::new().setup(|s| s.layout_hacks.expand_vertically = true);
        let modal_container_center = Container::new();
        let modal_container_overlay = Container::new();

        let hud = Column::new().setup(|c| {
            c.add(hud_player);

            // TODO: Put an extra container here, and swap its contents between the modal_container_side and the notifications pane
            c.add(modal_container_side.share());  
            c.add(Row::new().setup(|r| {
                r.add(hud_time);
                r.add(Spacer::new());
            }));
        });

        let g = globals.clone();
        let hud_target = Window::new().setup(|w| {
            w.set(Canvas::new().setup(|c| {
                c.set_draw(move |brush, menu| {
                    g.graphics.borrow_mut().draw_target_hud(&g, brush, menu)
                })
            }));
            w.window_border_override = Some(TARGET_WBORDER);
        });

        let g = globals.clone();
        io.menu(|out, menu: Menu| {
            let globals = g.clone();

            { 
                globals.graphics.borrow_mut().mouse_xy = Some(menu.mouse_xy()); 
            }

            let g = globals.clone();
            let game_rect = out.rect();
            menu.on_tick(move |_| { 
                // update graphics
                g.graphics.borrow_mut().pre_move_post_move_or_resize(&g, game_rect);

                g.npcs.pre_tick(&g);
                g.player.borrow_mut().on_tick(&g);
                g.graphics.borrow_mut().pre_move_post_move_or_resize(&g, game_rect);
                g.npcs.tick(&g);

                g.graphics.borrow_mut().post_tick_or_resize(&g, game_rect);

                Signal::Refresh
            });

            sitemode_display.draw(globals.ui.share(), out.brush(), menu.share());
            let hud_rect = rect(2, 2, 22.min(out.brush().rect().width() - 4), out.brush().rect().height() - 4);
            hud.draw(globals.ui.share(), out.brush().region(hud_rect), menu.share());

            if false {  // Only turn this on when targets are available
                let target_x1 = hud_rect.max_x();
                let target_rect = rect(hud_rect.max_x(), 2, out.brush().rect().max_x() - target_x1 - 2, 12.min(out.brush().rect().height() - 4));
                hud_target.draw(globals.ui.share(), out.brush().region(target_rect), menu.share());

                // TODO: Draw the modal_container_center here
            }

            // perform modal maintenance 
            {
                let g = globals.clone();
                let graphics = g.graphics.borrow();
                graphics.contextuals.update_widgets(modal_container_side.share(), modal_container_center.share(), modal_container_overlay.share());
            }

            modal_container_overlay.draw(globals.ui.share(), out.brush(), menu.share());
        });
    }

    fn calculate_viewport(&mut self, screen_boundaries: CellRect, player: &Player) {
        let ego_rect = rect(
            0, 0, 
            // TODO: Round up
            screen_boundaries.width() / SCCELL_X + 1, screen_boundaries.height() / SCCELL_Y + 1
        );

        self.viewport = if let Some(player_xy) = player.xy {
            Some(Viewport {
                rect: ego_rect,
                observer_in_rect: ego_rect.center().cast_unit(),
                observer: player_xy,
            })
        } else {
            None
        }
    }

    pub fn mouseover_view(&self) -> Option<GlobalView> {
        if let Some(mouse) = self.mouse_xy {
            if let Some(_) = self.viewport {
                return self.egosphere.at(point2(mouse.x / SCCELL_X, mouse.y / SCCELL_Y))
            }
        }
        None
    }
}
