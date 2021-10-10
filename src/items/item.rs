use std::{borrow::Cow};

#[derive(Clone)]
pub struct Item<Data> {
    pub profile: ItemProfile,
    pub data: Data,
}

#[derive(Clone, Debug)]
pub struct ItemProfile {
    pub name: Cow<'static, str>,
    pub icon: ItemIcon,
}

#[derive(Clone, Copy, Debug)]
pub struct ItemIcon {
    pub art: u16,  
    pub fg: u8,  
}

pub enum ItemDyn {
    Dakka(Dakka)
}

pub struct Dakka {
    // TODO: Projectile info!
}

impl Item<Dakka> {
    pub fn broad(self) -> Item<ItemDyn> {
        Item { 
            profile: self.profile,
            data: ItemDyn::Dakka(self.data)
        }
    }
}