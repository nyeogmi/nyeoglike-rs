mod block;
mod level;
mod room;

use moogle::{Id, OneToMany, Pom};

pub use block::Block;
pub use level::Level;
pub use room::Room;

pub struct Terrain {
    levels: Pom<Level>,
    rooms: Pom<Room>,

    level_rooms: OneToMany<Id<Level>, Id<Room>>,
}

impl Terrain {
    pub fn new() -> Terrain {
        Terrain { 
            levels: Pom::new(), 
            rooms: Pom::new(), 

            level_rooms: OneToMany::new(), 
        }
    }
}