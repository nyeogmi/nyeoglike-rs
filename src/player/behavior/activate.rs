use crate::reexports::*;
use crate::player::*;


pub struct Activate {
    queuing: bool,
    contextual_column: Option<Column>,
    contextual: Option<ContextualHandle>
}

impl std::fmt::Debug for Activate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Activate").field("queuing", &self.queuing).field("contextual", &self.contextual).finish()
    }
}

#[derive(Clone, Copy)]
pub struct ActivateToken;

impl Activate {
    pub(in crate::player) fn new() -> Activate {
        Activate {  
            queuing: false,
            contextual_column: None,
            contextual: None,
            // TODO: Allow a cooldown, or delay
        }
    }

    fn touched(&mut self, globals: &Rc<GlobalState>) {
        let graphics = globals.graphics.borrow();
        if let Some(view) = graphics.mouseover_view() {
            let widgets = context_widgets(globals, view.point());
            if widgets.len() > 0 {
                if let Some(contextual_column) = &self.contextual_column {
                    contextual_column.setup(|c| {
                        c.clear();
                        for w in widgets { c.add(w) }
                    });
                    return;
                }
            }
        }
        self.queuing = false;
    }
}

impl CanPerform<ActivateToken> for Player {
    fn get_activity_state(&self, _token: ActivateToken) -> ActivityState {
        if self.behavior.activate.queuing { return ActivityState::Queuing; }
        return ActivityState::Ready;
    }

    fn internal_mark_queuing(&mut self, _token: ActivateToken, queuing: bool) { 
        self.behavior.activate.queuing = queuing;
        if queuing {
            self.behavior.activate.contextual = None; // force it to be regenerated
        }
    }

    fn handle_auxiliary(&mut self, _token: ActivateToken, auxiliary: Auxiliary) -> bool { 
        // TODO: Tile selection? Beats me.
        if self.behavior.activate.queuing {
            // if the player moves, give up for now
            self.behavior.activate.queuing = false;
        }

        return false;
    }

    fn act(&mut self, globals: &Globals, _token: ActivateToken) -> bool {
        // NOTE: This currently has no effect on the player character
        let mut graphics = globals.graphics.borrow_mut();
        let mut activate = &mut self.behavior.activate;

        if activate.queuing {
            match activate.contextual {
                None => {
                    if let Some(view) = graphics.mouseover_view() {
                        // now generate one button for each item in the cell
                        let widgets = context_widgets(globals, view.point());
                        if widgets.len() > 0 {
                            let mouse_xy = graphics.mouse_xy.unwrap_or(point2(0, 0));
                            let column = Column::new().setup(|c| for w in widgets { c.add(w) });
                            activate.contextual_column = Some(column.share());
                            let ctx = generate_contextual(mouse_xy, column);

                            let ctx_id = graphics.contextuals.show(ctx);
                            activate.contextual.replace(ctx_id);
                            return false;
                        }
                    }
                    activate.queuing = false;
                    return false;
                }
                Some(handle) => {
                    if !graphics.contextuals.is_showing(handle) {
                        // somehow has been closed
                        activate.queuing = false;
                        activate.contextual_column = None;
                    }
                    return false;
                }
            }
        } else {
            match activate.contextual {
                None => {
                    // great! no work to do
                    return false
                }
                Some(handle) => {
                    if graphics.contextuals.is_showing(handle) {
                        graphics.contextuals.unshow(handle);
                        activate.contextual = None;
                        activate.contextual_column = None;
                    }
                    return false
                }
            }
        }
    }

    fn cooldown(&mut self, _globals: &Globals, _token: ActivateToken) {
        return
    }
}

fn context_widgets(globals: &Globals, point: GlobalPoint) -> SmallVec<[AnyWidget; 10]> {
    let cursor = globals.at(point);
    let mut widgets = SmallVec::new();
    for spawn in cursor.items().iter().rev() {
        if let Some(item) = globals.items.spawns.get(spawn) {
            let item_ref = item.borrow();
            widgets.push(Button::new().setup(|b| {
                b.text = item_ref.item.profile.name.to_string();
                let g: Globals = globals.clone();
                b.set_command(move |_, _, _| {
                    let mut player = g.player.borrow_mut();
                    player.grab(&g, spawn);
                    player.behavior.activate.touched(&g);
                    Signal::Refresh
                })
            }).into())
        }
    };
    widgets
}

// A normal number of click actions
fn generate_contextual(mouse_xy: CellPoint, column: Column) -> Contextual {
    let window = Window::new().setup(|w| w.set(column));
    let mouse_xy = mouse_xy;

    Contextual::Tooltip(mouse_xy, AnyWidget::wrap(window))
}