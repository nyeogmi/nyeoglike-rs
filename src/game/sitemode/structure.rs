use crate::game::reexports::*;

use super::{Intent, graphics::Memory};

pub struct SiteMode {
    pub(super) player_xy: Option<GlobalView>,
    pub(super) egosphere: Egosphere,

    pub(super) intent: Intent,
    pub(super) memory: Memory,
}

impl SiteMode {
    pub fn new() -> SiteMode {
        SiteMode {
            player_xy: None,
            egosphere: Egosphere::new(false),

            intent: Intent::new(),
            memory: Memory::new(),
        }
    }
}