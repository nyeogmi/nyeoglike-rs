use chiropterm::*;
use euclid::rect;

#[derive(Clone, Copy)]
pub struct WidgetDimensions {
    // widget will force its width below this if possible
    pub min: CellSize,
    pub preferred: CellSize,
    pub max: CellSize,
}

impl WidgetDimensions {
    pub(crate) fn fixup(&self) -> WidgetDimensions {
        // TODO: fix impossibilities
        *self
    }

    pub fn tailor<'a, X: Brushable>(&self, brush: Brush<'a, X>) -> Brush<'a, X> {
        // TODO: Make the brush region bigger if it's too small, or smaller if it's too big
        let existing_size = brush.rect().size;

        let region = brush.region(rect(
            0, 0,
            self.min.width.max(self.max.width.min(existing_size.width)),
            self.min.height.max(self.max.height.min(existing_size.height)),
        ));
        region
    }
}