use crate::game::reexports::*;

pub struct SiteMode {
    pub(super) player_xy: Option<GlobalView>,
    pub(super) egosphere: Egosphere,
}

impl SiteMode {
    pub fn new() -> SiteMode {
        SiteMode {
            player_xy: None,
            egosphere: Egosphere::new(false),
        }
    }
}