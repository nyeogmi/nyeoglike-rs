use crate::reexports::*;

pub(super) const EMPTY_FADE: (u8, u8) = (colors::Dark[1], colors::Light[3]);
pub(super) const EMPTY: (u8, u8) = (colors::Dark[0], colors::Light[3]);

pub(super) const TOP_FADE: (u8, u8) = (colors::Light[1], colors::Light[1]);
pub(super) const TOP: (u8, u8) = (colors::Light[3], colors::Light[1]);

pub(super) const SIDE_FADE: (u8, u8) = (colors::Dark[3], colors::Dark[0]);
pub(super) const SIDE: (u8, u8) = (colors::DkGreen[3], colors::LtGreen[1]);

pub(in crate) const SCCELL_X: isize = 2;
pub(in crate) const SCCELL_Y: isize = 2;