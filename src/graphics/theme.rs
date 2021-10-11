use chiroptui::look_and_feel::BaseTheme;

use crate::reexports::*;

use chiroptui::look_and_feel::*;

use super::constants;

pub const THEME: Theme = Theme {
    base: BaseTheme {
        wallpaper: (0, 0),  // not relevant
    },
    window: WindowTheme { 
        borders: WindowBorders::DOS {
            border: constants::FADE_FG,
            border_double: true,
            active_title_fg: colors::Light[2],
            inactive_title_fg: colors::Light[0],
        }, 
        color: (colors::Dark[0], colors::Light[2]),
    },
    button: ButtonTheme { 
        bevel: (colors::Dark[0], colors::Dark[3]), 
        color: (colors::Dark[0], colors::Light[3]), 
        preclick: (colors::Dark[1], colors::Light[3]),
    },
    input_box: InputBoxTheme { 
        bevel: (colors::DkGreen[0], colors::DkGreen[3]), 
        deselected: (colors::DkGreen[0], colors::Light[2]), 
        selected: (colors::DkGreen[1], colors::Light[2]), 
        preclick: (colors::DkOrange[1], colors::Light[3]), 
        cursor: (colors::LtOrange[1], colors::Light[3]),
    }
};

pub const TARGET_WBORDER: WindowBorders = WindowBorders::DOS {
    border: constants::FADE_FG,
    border_double: false,
    active_title_fg: colors::Light[2],
    inactive_title_fg: colors::Light[0],
};