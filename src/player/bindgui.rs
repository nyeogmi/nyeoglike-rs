use crate::reexports::*;
use crate::player::*;

impl Player {
    pub fn add_basic_controls<'frame>(&self, globals: &'_ Globals, menu: WidgetMenu<'frame, CanvasState>) {
        let g = globals.clone();
        menu.on_key(OnKey::only(Keycode::W).up_or_down(), move |_, _, k| {
            g.sitemode.borrow_mut().input_auxiliary(Auxiliary::Up(k.is_down()));
            Signal::Continue
        });

        let g = globals.clone();
        menu.on_key(OnKey::only(Keycode::S).up_or_down(), move |_, _, k| {
            g.sitemode.borrow_mut().input_auxiliary(Auxiliary::Down(k.is_down()));
            Signal::Continue
        });

        let g = globals.clone();
        menu.on_key(OnKey::only(Keycode::A).up_or_down(), move |_, _, k| {
            g.sitemode.borrow_mut().input_auxiliary(Auxiliary::Left(k.is_down()));
            Signal::Continue
        });

        let g = globals.clone();
        menu.on_key(OnKey::only(Keycode::D).up_or_down(), move |_, _, k| {
            g.sitemode.borrow_mut().input_auxiliary(Auxiliary::Right(k.is_down()));
            Signal::Continue
        });

        let g = globals.clone();
        menu.on_key(OnKey::only(Keycode::Space).pressed(), move |_, _, _| {
            g.sitemode.borrow_mut().input_press_charge();
            Signal::Continue
        });
    }
}