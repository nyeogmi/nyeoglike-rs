use super::reexports::*;

pub const ASPECT_CONFIG: AspectConfig = AspectConfig {
    pref_min_term_size: size2(80, 50),  // but expect ~112x60
    pref_max_term_size: size2(256, 256),
};