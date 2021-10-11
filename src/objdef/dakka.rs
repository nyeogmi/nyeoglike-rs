use crate::reexports::*;


pub const ITEM_SHOTGUN: Item<Dakka> = Item {
    profile: ItemProfile {
        name: Cow::Borrowed("shotgun"),
        icon: ItemIcon { art: b'=' as u16, fg: colors::LtFuchsia[1] }
    },
    data: Dakka {

    }
};
pub const ITEM_PISTOL: Item<Dakka> = Item {
    profile: ItemProfile {
        name: Cow::Borrowed("pistol"),
        icon: ItemIcon { art: b'\'' as u16, fg: colors::LtFuchsia[1] }
    },
    data: Dakka {

    }
};