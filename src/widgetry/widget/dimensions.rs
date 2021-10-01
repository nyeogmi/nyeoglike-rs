use chiropterm::*;
use euclid::{rect, size2};

#[derive(Clone, Copy)]
pub struct WidgetDimensions {
    // widget will force its width below this if possible
    pub min: CellSize,
    pub preferred: CellSize,
    pub max: CellSize,
}

impl WidgetDimensions {
    pub fn bogus() -> WidgetDimensions {
        WidgetDimensions {
            min: size2(0, 0),
            preferred: size2(0, 0),
            max: size2(0, 0),
        }
    }

    pub(crate) fn fixup(&self) -> WidgetDimensions {
        // TODO: fix impossibilities
        *self
    }

    pub fn tailor<'a>(&self, brush: Brush<'a>) -> Brush<'a> {
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