use crate::reexports::*;

pub(super) const FADE_FG: u8 = colors::DkGreen[2];

pub(super) const EMPTY_FADE: (u8, u8) = (colors::DkGreen[0], colors::Light[3]);
pub(super) const EMPTY: (u8, u8) = (colors::Dark[0], colors::Light[3]);

pub(super) const TOP_FADE: (u8, u8) = (colors::DkGreen[0], FADE_FG);
pub(super) const TOP: (u8, u8) = (colors::Dark[0], colors::LtOrange[0]);

pub(super) const SIDE_FADE: (u8, u8) = (colors::DkGreen[0], FADE_FG);
pub(super) const SIDE: (u8, u8) = (colors::Dark[0], colors::LtGreen[3]);

pub(in crate) const SCCELL_X: isize = 2;
pub(in crate) const SCCELL_Y: isize = 2;