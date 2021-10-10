use crate::reexports::*;


pub const ITEM_SHOTGUN: Item<Dakka> = Item {
    profile: ItemProfile {
        name: Cow::Borrowed("shotgun"),
        icon: ItemIcon { art: b'=' as u16, fg: colors::Light[1] }
    },
    data: Dakka {

    }
};