use crate::reexports::*;

pub struct Room {
    blocks: RefCell<CopyEndlessGrid<Block, RoomSpace>>,
    // TODO: Items, NPC spawns, wallpaper
}

impl Room {
    pub(super) fn new() -> Room {
        Room { blocks: RefCell::new(CopyEndlessGrid::new(Block::Plain)) }
    }

    pub fn get(&self, p: RoomPoint) -> Block {
        self.blocks.borrow().get(p)
    }

    pub fn set(&self, p: RoomPoint, b: Block) {
        self.blocks.borrow_mut().set(p, b)
    }
}