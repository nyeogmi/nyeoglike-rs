mod item;

use crate::reexports::*;
pub use item::*;

pub struct Items {
    pub spawns: FloatingPom<ItemSpawn>,

    pub location_of: ManyToOne<Id<ItemSpawn>, GlobalPoint>,
}

pub struct ItemSpawn {
    pub item: Item<ItemDyn>
}

impl Items {
    pub fn new() -> Items {
        Items {
            spawns: FloatingPom::new(),

            location_of: ManyToOne::new(),
        }
    }

    pub fn spawn_item_raw(&self, location: GlobalPoint, item: Item<ItemDyn>) -> Id<ItemSpawn> {
        let id = self.spawns.insert(ItemSpawn { item });
        self.location_of.fwd().insert(id, location);
        return id
    }

    pub fn take_item(&self, spawn: Id<ItemSpawn>) -> Option<Item<ItemDyn>> {
        if let Some(actual_spawn) = self.spawns.get(spawn) {
            let item = actual_spawn.borrow().item.clone();
            self.spawns.remove(spawn);
            Some(item)
        } else {
            None
        }
    }
}