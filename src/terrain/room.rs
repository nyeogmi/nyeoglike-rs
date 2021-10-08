use crate::reexports::*;

pub struct Room {
    blocks: CopyEndlessGrid<Block, RoomSpace>,
    // TODO: Items, NPC spawns, wallpaper
}

impl Room {
    pub(super) fn new() -> Room {
        Room { blocks: CopyEndlessGrid::new(Block::Plain) }
    }

    pub fn get(&self, p: RoomPoint) -> Block {
        self.blocks.get(p)
    }

    pub fn set(&mut self, p: RoomPoint, b: Block) {
        self.blocks.set(p, b)
    }
}