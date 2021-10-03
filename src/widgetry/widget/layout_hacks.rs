use super::WidgetDimensions;

#[derive(Clone, Copy)]
pub struct LayoutHacks {
    pub expand_horizontally: bool,
    pub expand_vertically: bool,
}

impl LayoutHacks {
    pub fn new() -> LayoutHacks {
        LayoutHacks {
            expand_horizontally: false,
            expand_vertically: false,
        }
    }

    pub fn apply(&self, mut wd: WidgetDimensions) -> WidgetDimensions {
        if self.expand_horizontally { 
            wd.horizontal_spacer_count = wd.horizontal_spacer_count.max(1); 
        };
        if self.expand_vertically { 
            wd.vertical_spacer_count = wd.vertical_spacer_count.max(1); 
        };
        wd
    }
}