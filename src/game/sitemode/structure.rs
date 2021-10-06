use crate::game::reexports::*;

use super::Intent;

pub struct SiteMode {
    pub(super) player_xy: Option<GlobalView>,
    pub(super) egosphere: Egosphere,

    pub(super) intent: Intent,
}

impl SiteMode {
    pub fn new() -> SiteMode {
        SiteMode {
            player_xy: None,
            egosphere: Egosphere::new(false),

            intent: Intent::new(),
        }
    }
}