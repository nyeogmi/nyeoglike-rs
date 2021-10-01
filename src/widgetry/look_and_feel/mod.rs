mod w95;

pub use w95::W95Args;

#[derive(Clone, Copy)]
pub struct Theme {
    pub base: BaseTheme,
    pub window: WindowTheme,
    pub button: ButtonTheme,
    pub input_box: InputBoxTheme,
}

#[derive(Clone, Copy)]
pub struct BaseTheme {
    pub wallpaper: (u8, u8)
}

#[derive(Clone, Copy)]
pub struct WindowTheme {
    pub bevel: (u8, u8),
    pub color: (u8, u8),
}

#[derive(Clone, Copy)]
pub struct ButtonTheme {
    pub bevel: (u8, u8),
    pub color: (u8, u8),
    pub preclick: (u8, u8),
}

#[derive(Clone, Copy)]
pub struct InputBoxTheme {
    pub bevel: (u8, u8),
    pub deselected: (u8, u8),
    pub selected: (u8, u8),
    pub preclick: (u8, u8),
    pub cursor: (u8, u8),
}