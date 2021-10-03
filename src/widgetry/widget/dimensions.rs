use chiropterm::*;
use euclid::{rect, size2};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WidgetDimensions {
    // widget will force its width below this if possible
    pub min: CellSize,
    pub preferred: CellSize,
    pub max: CellSize,
    pub align_size_to: CellSize,
    pub horizontal_spacer_count: usize,
    pub vertical_spacer_count: usize,
}

impl WidgetDimensions {
    pub const fn zero() -> WidgetDimensions {
        WidgetDimensions {
            min: size2(0, 0),
            preferred: size2(0, 0),
            max: size2(0, 0),
            align_size_to: size2(1, 1),
            horizontal_spacer_count: 0,
            vertical_spacer_count: 0,
        }
    }

    pub(crate) fn fixup(mut self) -> WidgetDimensions {
        // TODO: fix impossibilities
        self = self.shape_to_align(self.align_size_to);
        self
    }

    pub fn tailor<'a>(&self, brush: Brush<'a>) -> Brush<'a> {
        let existing_size = brush.rect().size;

        let region = brush.region(rect(
            0, 0,
            existing_size.width - existing_size.width % self.align_size_to.width,
            existing_size.height - existing_size.height % self.align_size_to.height,
        ));

        let existing_size = brush.rect().size;

        let region = region.region(rect(
            0, 0,
            self.min.width.max(self.max.width.min(existing_size.width)),
            self.min.height.max(self.max.height.min(existing_size.height)),
        ));
        region
    }

    pub(crate) fn shape_to_align(mut self, align: CellSize) -> WidgetDimensions {
        fn fix(align: CellSize, mut sz: CellSize) -> CellSize {
            if align.width > 0 && sz.width % align.width != 0 {
                sz.width += align.width - (sz.width % align.width);
            }
            if align.height > 0 && sz.height % align.height != 0 {
                sz.height += align.height - (sz.height % align.height);
            }
            sz
        }

        self.min = fix(align, self.min);
        self.preferred = fix(align, self.preferred);
        self.max = fix(align, self.max);

        self
    }

    pub(crate) fn increase(mut self, amt: CellSize) -> WidgetDimensions {
        self.min.width += amt.width;
        self.min.height += amt.height;

        self.preferred.width += amt.width;
        self.preferred.height += amt.height;

        self.max.width += amt.width;
        self.max.height += amt.height;

        self
    }
}