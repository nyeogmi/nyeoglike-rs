use bresenham::Bresenham;

use crate::reexports::*;

pub struct Hotline {
    pub internal_facing: Cardinal,
    pub viewdist_forward: usize,
    pub side_distance: usize,
    pub mandatory_steps_forward: usize,
}

#[derive(Clone, Copy, Debug)]
struct PointClearances {
    left: usize, right: usize,

    forward: usize, // back: usize,
}

#[derive(Clone, Copy, Debug)]
struct RoomClearances {
    left: usize, right: usize,

    forward: usize, // back: usize,
}

fn point_clearances(
    start: EgoPoint,
    facing: Cardinal,
    viewdist_forward: usize,
    viewdist_sides: usize,
    blocked: impl Fn(EgoPoint) -> bool, 
) -> PointClearances {
    let mut max_right = 0;
    let mut max_left = 0;
    let mut max_forward = 0;
    for right in 1..=viewdist_sides {
        if blocked(start + facing.right().offset_by(right as isize)) { break; }
        max_right = right;
    }
    for left in 1..=viewdist_sides {
        if blocked(start + facing.left().offset_by(left as isize)) { break; }
        max_left = left;
    }
    for forward in 1..=viewdist_forward {
        if blocked(start + facing.offset_by(forward as isize)) { break; }
        max_forward = forward;
    }

    PointClearances {
        left: max_left,
        right: max_right,
        forward: max_forward,
    }
}

fn room_clearances(
    start: EgoPoint,
    facing: Cardinal,
    viewdist_forward: usize,
    viewdist_sides: usize,
    blocked: impl Fn(EgoPoint) -> bool, 
) -> (PointClearances, RoomClearances) {
    let base_clearances = point_clearances(start, facing, viewdist_forward, viewdist_sides, |x| blocked(x));
    let mut min_left = base_clearances.left;
    let mut min_right = base_clearances.right;

    let practical_dist = viewdist_forward.min(base_clearances.forward);
    for i in 1..=practical_dist {
        let new_start = start + facing.offset_by(i as isize);
        assert!(!blocked(new_start));
        let clearances_here = point_clearances(new_start, facing, 0, viewdist_sides, |x| blocked(x));
        min_left = min_left.min(clearances_here.left);
        min_right = min_right.min(clearances_here.right);
    }

    (base_clearances, RoomClearances {
        left: min_left,
        right: min_right,
        forward: base_clearances.forward,
    })
}

impl Hotline {
    pub(in crate::npcs) fn advance(&mut self, blocked: impl Fn(EgoPoint) -> bool, mut turn: impl FnMut(Egocentric), step: impl FnOnce(EgoVec)) {
        if self.mandatory_steps_forward > 0 {
            self.mandatory_steps_forward -= 1;
            step(self.internal_facing.offset());
            return;
        }

        // look for doors the way we would look ahead: use forward for both
        let (point, room) = room_clearances(point2(0, 0), self.internal_facing, self.viewdist_forward, self.viewdist_forward, |x| blocked(x));

        // calculate whether we were already on the side
        let mut on_side = true;
        for dist in [self.side_distance+1] {
            let prev_point: EgoPoint = point2(0, 0) + self.internal_facing.offset_by(-(dist as isize));
            if blocked(prev_point) { on_side = false; break; }
            let clearance = point_clearances(prev_point, self.internal_facing, self.viewdist_forward, self.viewdist_forward, |x| blocked(x));
            if clearance.right > self.side_distance {
                on_side = false; break;
            }
        }
        if on_side {
            for dist in 1..self.side_distance+1 {
                let prev_point: EgoPoint = point2(0, 0) + self.internal_facing.offset_by(-(dist as isize)) + self.internal_facing.right().offset_by(self.side_distance as isize);
                if blocked(prev_point) {
                    on_side = false; break;
                }
            }
        }

        println!("point, room, on_side: {:?} {:?} {:?}", point, room, on_side);

        if on_side && point.right > self.side_distance {
            // door!!!!
            let (door_point, door_room) = room_clearances(
                point2(0, 0), self.internal_facing.right(), self.viewdist_forward, self.viewdist_forward, |x| blocked(x)
            );
            println!("door: {:?}", door_room);

            if door_room.left >= 1 && door_room.right >= 1 || door_room.left == 0 && door_room.right == 0 {
                self.internal_facing = self.internal_facing.right();
                self.mandatory_steps_forward = self.side_distance;
                return;
            }
        }

        if point.forward <= self.side_distance {
            println!("too close: turning");
            self.internal_facing = self.internal_facing.left();
            return;
        }

        step(self.internal_facing.offset());
    }
}