use std::iter;

use bresenham::Bresenham;

use crate::reexports::*;

use super::{Behavior};

pub struct Player {
    pub xy: Option<GlobalView>,
    pub cumulative_xy_shift: EgoVec,

    // behavior
    pub behavior: Behavior,
}

impl Player {
    pub fn new() -> Player {
        Player {
            xy: None,
            cumulative_xy_shift: vec2(0, 0),

            behavior: Behavior::new(),
        }
    }

    pub fn move_by(&mut self, globals: &Globals, offset: EgoVec, flail: bool) -> EgoVec {
        let mut sum = vec2(0, 0);
        let mut last_point = vec2(0, 0);

        for (x, y) in Bresenham::new((0, 0), (offset.x, offset.y)).chain(iter::once((offset.x, offset.y))) {
            let point = vec2(x, y);
            if point == vec2(0, 0) { continue }
            let suboffset = point - last_point;
            last_point = point;

            sum += self.move_by_1(globals, suboffset, flail)
        }
        // TODO: Take memory at every intermediate point?
        self.cumulative_xy_shift += sum;
        sum
    }

    pub fn move_by_1(&mut self, globals: &Globals, offset: EgoVec, flail: bool) -> EgoVec {
        if let Some(player_xy) = self.xy {
            let options = if flail { 
                neighbors(offset) 
            } else { 
                let mut offset_x0 = offset; offset_x0.x = 0;
                let mut offset_y0 = offset; offset_y0.y = 0;

                [offset, offset_x0, offset_y0] 
            };

            for (i, &thing_to_try) in options.iter().enumerate() {
                if thing_to_try == offset && i != 0 {
                    continue;  // don't try offset more than once
                }

                let new_xy = globals.terrain.step_offset(player_xy, thing_to_try);
                let at_new_xy = globals.at(new_xy.point());
                if at_new_xy.is_blocked() {
                    continue;
                }

                self.xy = self.xy.map(|pxy| globals.terrain.step_offset(pxy, thing_to_try));
                return thing_to_try;
            }
            return EgoVec::zero();
        } else {
            return EgoVec::zero();
        }
    }
}

fn neighbors(offset: EgoVec) -> [EgoVec; 3] {
    match (offset.x, offset.y) {
        ( 0,  0) => [offset, vec2( 0,   0), vec2( 0,  0)],

        ( 0, -1) => [offset, vec2(-1, -1), vec2( 1, -1)],
        ( 0,  1) => [offset, vec2(-1,  1), vec2( 1,  1)],

        (-1,  0) => [offset, vec2(-1, -1), vec2( -1, 1)],
        ( 1,  0) => [offset, vec2( 1, -1), vec2(  1, 1)],

        (-1, -1) => [offset, vec2(-1,  0), vec2( 0, -1)],
        ( 1, -1) => [offset, vec2( 1,  0), vec2( 0, -1)],
        (-1,  1) => [offset, vec2(-1,  0), vec2( 0,  1)],
        ( 1,  1) => [offset, vec2( 1,  0), vec2( 0,  1)],

        _ => panic!("bad input from bresenham"),
    }
}
