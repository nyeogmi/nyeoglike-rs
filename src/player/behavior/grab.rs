use crate::reexports::*;

impl Player {
    pub fn grab(&mut self, globals: &Globals, spawn: Id<ItemSpawn>) {
        let item = globals.items.take_item(spawn);
        println!("i grabbed it!!! {:?}", item);
        // TODO: Actually grab the item
    }
}