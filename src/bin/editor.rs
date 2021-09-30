use chiropterm::*;
use euclid::*;
use moogle::Id;
use nyeoglike::terrain::{Room, Terrain};

const ASPECT_CONFIG: AspectConfig = AspectConfig {
    pref_min_term_size: size2(64, 48),  // but expect ~112x60
    pref_max_term_size: size2(256, 256),
};

struct EditorState {
    terrain: Terrain,
    room: Id<Room>,
    cursor: Point2D<isize, isize>
}

fn main() {
    nyeoglike::editor::main();
}