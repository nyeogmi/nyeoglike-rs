use crate::reexports::*;

pub struct Hotline {
    pub internal_facing: Cardinal,
}

impl Hotline {
    pub(in crate::npcs) fn advance(&mut self, blocked: impl Fn(EgoPoint) -> bool, step: impl FnOnce(EgoVec)) {
        let right = point2(0, 0) + self.internal_facing.right().offset();
        let back_right = point2(0, 0) + self.internal_facing.right().offset() + self.internal_facing.offset_by(-1);

        if blocked(back_right) && !blocked(right) {
            self.internal_facing = self.internal_facing.right();
            step(self.internal_facing.offset());
            return
        }

        let fwd = point2(0, 0) + self.internal_facing.offset();

        if blocked(fwd) {
            self.internal_facing = self.internal_facing.left()
        } else {
            step(self.internal_facing.offset())
        }
    }
}