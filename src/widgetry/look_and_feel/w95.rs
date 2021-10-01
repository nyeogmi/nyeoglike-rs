use chiropterm::colors::*;

use super::*;

pub struct W95Args {
    wallpaper: (u8, u8),
    inset: (u8, u8),
    window: (u8, u8),
    accent_preclick: (u8, u8),
    accent_subselected: (u8, u8),

    // "enclaves" include text boxes
    enclave: (u8, u8),
}

impl Theme {
    pub const W95: Theme = Theme::w95(
        W95Args {
            wallpaper: (Green[0], Dark[0]),
            inset: (Dark[0], Light[3]),
            window: (Light[2], Dark[0]),
            accent_preclick: (Green[2], Dark[0]),
            accent_subselected: (Orange[2], Dark[0]),
            enclave: (Light[3], Dark[0]),
        }
    );

    pub const W95_DARK: Theme = Theme::w95(
        W95Args {
            wallpaper: (Orange[0], Dark[0]),
            inset: (Dark[0], Light[0]),
            window: (Dark[1], Light[2]),
            accent_preclick: (Blue[0], Light[2]),
            accent_subselected: (Orange[2], Light[2]),

            enclave: (Dark[0], Light[2]),
        }
    );

    pub const W95_FRUITY: Theme = Theme::w95(
        W95Args {
            wallpaper: (Dark[2], Dark[0]),
            inset: (Dark[1], Purple[3]),
            window: (Purple[1], Light[2]),
            accent_preclick: (Cyan[0], Light[2]),
            accent_subselected: (Orange[0], Light[2]),
            enclave: (Purple[0], Light[2]),
        }
    );

    pub const fn w95(args: W95Args) -> Theme {
        let wallpaper = args.wallpaper;
        let inset = args.inset;
        let outset = (inset.1, inset.0);
        let window = args.window;
        let accent_preclick = args.accent_preclick;
        let accent_subselected = args.accent_subselected;
        let enclave = args.enclave;

        Theme {
            base: BaseTheme { 
                wallpaper: wallpaper
            },
            window: WindowTheme {
                bevel: outset,
                color: window,
            },
            button: ButtonTheme { 
                bevel: outset,
                color: window,  // TODO: This is completely wrong!
                preclick: accent_preclick,
            },
            input_box: InputBoxTheme {
                bevel: inset,
                deselected: enclave,
                selected: enclave,
                preclick: accent_preclick,
                cursor: accent_subselected,
            },
        }
    }
}