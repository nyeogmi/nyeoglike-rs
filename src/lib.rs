mod constants;
mod entry_point;
mod globals;
mod graphics;
mod items;
mod npcs;
#[allow(dead_code)] pub mod objdef;
mod player;
mod reexports;
mod terrain;

pub use entry_point::main;