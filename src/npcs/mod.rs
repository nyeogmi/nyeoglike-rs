use crate::reexports::*;

pub struct NPCs {
    pub table: Pom<NPC>,
    pub location_of: OneToOne<Id<NPC>, GlobalPoint>,
}

impl NPCs {
    pub fn new() -> NPCs {
        NPCs {  
            table: Pom::new(),
            location_of: OneToOne::new(),
        }
    }

    pub fn create_npc(&mut self) -> Id<NPC> {
        self.table.insert(NPC {

        })
    }
}

pub struct NPC {
}