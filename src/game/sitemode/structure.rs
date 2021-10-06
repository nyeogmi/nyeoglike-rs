use crate::game::reexports::*;

use super::{Intent, graphics::Memory};

pub struct SiteMode {
    // behavior
    pub(super) player_xy: Option<GlobalView>,
    pub(super) intent: Intent,

    // graphics
    pub(super) egosphere: Egosphere,
    pub(super) memory: Memory,
}

impl SiteMode {
    pub fn new() -> SiteMode {
        SiteMode {
            player_xy: None,
            intent: Intent::new(),

            egosphere: Egosphere::new(false),
            memory: Memory::new(),
        }
    }
}