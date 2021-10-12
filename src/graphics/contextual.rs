use crate::reexports::*;

pub struct ContextualServer {
    last_handle: u64,

    side: Option<AnyWidget>,
    side_handle: u64,

    center_state: CenterState,
    center: Option<AnyWidget>,
    center_handle: u64,
}

#[derive(Clone, Copy, Debug)]
pub struct ContextualHandle(u64);

pub enum Contextual {
    Side(AnyWidget),
    Tooltip(CellPoint, AnyWidget),
    // TODO: Tooltip
    // TODO: Side _and_ center
    // TODO: Full
}

enum CenterState {
    // TODO: Center,
    Tooltip(CellPoint),
}

impl ContextualServer {
    pub(super) fn new() -> ContextualServer {
        ContextualServer { 
            last_handle: 0,
            side: None, side_handle: 0,
            center_state: CenterState::Tooltip(point2(0, 0)), center: None, center_handle: 0,
        }
    }
    
    pub(super) fn update_widgets(&self, side: Container, center: Container, overlay: Container) {
        match &self.side {
            None => {
                side.setup(|s| { s.set(Spacer::new()) });
            }
            Some(w) => {
                let w2 = w.share();
                side.setup(|s| { s.set(w2) });
            }
        }

        match &self.center {
            None => {
                center.setup(|s| { s.set(Spacer::new() )});
                overlay.setup(|s| { s.set(Spacer::new() )});
            }
            Some(w) => {
                match self.center_state {
                    CenterState::Tooltip(v) => { 
                        let w2 = w.share();
                        center.setup(|s| { s.set(Spacer::new() )});
                        overlay.setup(|s| { 
                            s.set(BulletinBoard::new().setup(|bb| {
                                bb.add(v - vec2(2, 2), w2)
                            })) 
                        });
                    }
                }
            }
        }
    }

    pub fn show(&mut self, contextual: Contextual) -> ContextualHandle {
        self.last_handle += 1;
        let handle = self.last_handle;

        match contextual {
            Contextual::Side(aw) => {
                self.side = Some(aw);
                self.side_handle = handle;
            }
            Contextual::Tooltip(v, aw) => {
                self.center_state = CenterState::Tooltip(v);
                self.center = Some(aw);
                self.center_handle = handle;
            }
        }

        ContextualHandle(handle)
    }

    pub fn is_showing(&mut self, contextual_handle: ContextualHandle) -> bool {
        (self.side.is_some() && self.side_handle == contextual_handle.0) || 
        (self.center.is_some() && self.center_handle == contextual_handle.0) 
    }

    pub fn unshow(&mut self, contextual_handle: ContextualHandle) -> bool {
        if self.side.is_some() && self.side_handle == contextual_handle.0 {
            self.side = None;
            return true
        }
        if self.center.is_some() && self.center_handle == contextual_handle.0 {
            self.center = None;
            return true
        }
        return false
    }
}