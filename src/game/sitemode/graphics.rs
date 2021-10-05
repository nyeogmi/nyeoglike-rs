use crate::game::reexports::*;

impl SiteMode {
    pub fn draw<'frame>(&self, brush: Brush, menu: WidgetMenu<'frame, CanvasState>) {
        brush.putfs("GOT YOU!");
    }
}