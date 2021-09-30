use areaportal2d::{GlobalView as GV, GlobalPoint as GP};
use euclid::{Point2D, Rect, Size2D, Vector2D};

use crate::terrain::Room;

pub type GlobalView = GV<Room>;
pub type GlobalPoint = GP<Room>;

pub use areaportal2d::{
    EgoSpace, EgoPoint, EgoSize, EgoVec, EgoRect
};

pub struct RoomSpace;
pub type RoomPoint = Point2D<isize, RoomSpace>;
pub type RoomSize = Size2D<isize, RoomSpace>;
pub type RoomVec = Vector2D<isize, RoomSpace>;
pub type RoomRect = Rect<isize, RoomSpace>;