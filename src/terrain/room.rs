use crate::geom::*;
use gridd_euclid::{CopyEndlessGrid};

use super::Block;

pub struct Room {
    blocks: CopyEndlessGrid<RoomPoint, Block>,
    // TODO: Items, NPC spawns, wallpaper
}