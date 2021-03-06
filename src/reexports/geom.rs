use areaportal2d::{AreaPortal as Ap, Egosphere as Es, GlobalView as GV, GlobalPoint as GP, Viewport as Vp, Portals as Po};
use euclid::{Point2D, Rect, Size2D, Vector2D};
use moogle::Id;

use crate::terrain::Room;

pub type AreaPortal = Ap<Id<Room>>;
pub type Egosphere = Es<Id<Room>>;
pub type GlobalView = GV<Id<Room>>;
pub type GlobalPoint = GP<Id<Room>>;
pub type Viewport = Vp<Id<Room>>;
pub type Portals = Po<Id<Room>>;

pub use areaportal2d::{
    EgoSpace, EgoPoint, EgoSize, EgoVec, EgoRect
};

pub struct RoomSpace;
pub type RoomPoint = Point2D<isize, RoomSpace>;
#[allow(dead_code)]
pub type RoomSize = Size2D<isize, RoomSpace>;
#[allow(dead_code)]
pub type RoomVec = Vector2D<isize, RoomSpace>;
#[allow(dead_code)]
pub type RoomRect = Rect<isize, RoomSpace>;