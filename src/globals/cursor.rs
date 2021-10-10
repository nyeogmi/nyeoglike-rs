use crate::reexports::*;

pub struct Cursor<'a> {
    // NOTE: We just use a ref because these really, really shouldn't be kept around forever
    pub(in crate::globals) globals: &'a GlobalState,
    pub(in crate::globals) point: GlobalPoint,
}

impl<'a> Cursor<'a> {
    pub fn get_block(&self) -> Block {
        self.globals.terrain.get_block_raw(self.point)
    }

    pub fn set_block(&self, block: Block) {
        self.globals.terrain.set_block_raw(self.point, block)
    }

    pub(crate) fn is_blocked(&self) -> bool {
        self.get_block().is_blocked()
    }
}