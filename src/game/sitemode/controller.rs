use crate::game::reexports::*;

impl SiteMode {
    pub fn add_basic_controls<'frame>(&self, globals: &'_ Globals, menu: WidgetMenu<'frame, CanvasState>) {
        let g = globals.clone();
        menu.on_key(OnKey::only(Keycode::W).up_or_down(), move |_, _, k| {
            g.sitemode.borrow_mut().intent.walk.up = k.is_down();
            Signal::Continue
        });

        let g = globals.clone();
        menu.on_key(OnKey::only(Keycode::S).up_or_down(), move |_, _, k| {
            g.sitemode.borrow_mut().intent.walk.down = k.is_down();
            Signal::Continue
        });

        let g = globals.clone();
        menu.on_key(OnKey::only(Keycode::A).up_or_down(), move |_, _, k| {
            g.sitemode.borrow_mut().intent.walk.left = k.is_down();
            Signal::Continue
        });

        let g = globals.clone();
        menu.on_key(OnKey::only(Keycode::D).up_or_down(), move |_, _, k| {
            g.sitemode.borrow_mut().intent.walk.right = k.is_down();
            Signal::Continue
        });
    }
}