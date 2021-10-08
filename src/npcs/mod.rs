use bresenham::Bresenham;

use crate::reexports::*;

pub struct NPCs {
    pub table: Pom<NPC>,

    // TODO: Don't let people use this directly (so we can force an egosphere recalc)
    pub location_of: OneToOne<Id<NPC>, GlobalPoint>,
}

impl NPCs {
    pub fn new() -> NPCs {
        NPCs {  
            table: Pom::new(),
            location_of: OneToOne::new(),
        }
    }

    pub fn create_npc(&mut self, facing: Cardinal, move_ai: MoveAI, view_radius: isize) -> Id<NPC> {
        self.table.insert(NPC {
            facing,
            move_ai,
            view_radius,
            egosphere: Egosphere::new(false),
        })
    }

    pub fn pre_tick(&mut self, globals: &Globals) {
        let terrain = globals.terrain.borrow_mut();
        for (id, npc) in self.table.iter_mut() {
            if let Some(loc) = self.location_of.fwd().get(id) {
                let viewport = Viewport { 
                    rect: rect(-npc.view_radius, -npc.view_radius, 2 * npc.view_radius + 1, 2 * npc.view_radius + 1),
                    observer_in_rect: point2(0, 0),
                    observer: loc.facing(npc.facing),
                };
                terrain.recalculate_egosphere(&mut npc.egosphere, viewport)
            }
        }
    }

    pub fn tick(&mut self, globals: &Globals) {
        let location_of = &mut self.location_of;
        for (id, npc) in self.table.iter_mut() {
            if let Some(loc) = location_of.fwd().get(id) {
                let mut tmp_move_ai = MoveAI::Idle;
                let facing = Cell::new(npc.facing);
                mem::swap(&mut npc.move_ai, &mut tmp_move_ai);

                let terr = globals.terrain.borrow_mut();
                tmp_move_ai.advance(
                    |xy| {
                        if let Some(g) = npc.egosphere.at(xy) {
                            return terr.get(g.point()).is_blocked();
                        }
                        return true;
                    },
                    |ego| {
                        facing.replace(facing.get().rotated(ego));
                    },
                    |amt| {
                        location_of.fwd().insert(id, terr.step_offset(loc.facing(facing.get()), amt).point());
                    },
                );

                mem::swap(&mut npc.move_ai, &mut tmp_move_ai);
                npc.facing = facing.get();
            }
        }
    }
}

pub struct NPC {
    pub facing: Cardinal,
    pub move_ai: MoveAI,
    pub view_radius: isize,
    pub egosphere: Egosphere,
}

pub enum MoveAI {
    Idle,
    Hotline(Hotline)
}

// hotline miami-style "hug the right wall" AI

impl MoveAI {
    pub fn advance(&mut self, blocked: impl Fn(EgoPoint) -> bool, turn: impl FnMut(Egocentric), step: impl FnOnce(EgoVec)) {
        match self {
            MoveAI::Idle => {}
            MoveAI::Hotline(h) => h.advance(blocked, turn, step)
        }
    }
}

pub struct Hotline {
    pub distance: isize,
    pub internal_facing: Cardinal,
}

impl Hotline {
    fn advance(&mut self, blocked: impl Fn(EgoPoint) -> bool, mut turn: impl FnMut(Egocentric), step: impl FnOnce(EgoVec)) {
        let d = self.distance;
        let centering = |start: EgoPoint, facing: Cardinal| {
            let mut min_x = 0;
            let mut max_x = 0;
            let mut min_y = 0;
            let mut max_y = 0;

            for x0 in 0..=d {
                if blocked(start + vec2(x0, 0)) { break; }
                max_x = x0;
            }
            for x0 in 0..=d {
                if blocked(start + vec2(-x0, 0)) { break; }
                min_x = x0;
            }
            for y0 in 0..=d {
                if blocked(start + vec2(0, y0)) { break; }
                max_y = y0;
            }
            for y0 in 0..=d {
                if blocked(start + vec2(0, -y0)) { break; }
                min_y = y0;
            }

            match facing {
                Cardinal::North => (min_x, max_x),
                Cardinal::South => (max_x, min_x),
                Cardinal::East => (min_y, max_y),
                Cardinal::West => (max_y, min_y),
            }
        };

        let mut facing = self.internal_facing;
        let mut min_clearance_left = self.distance;
        let mut min_clearance_right = self.distance;
        for point in Bresenham::new((0, self.distance + 1), (0, -(self.distance + 1))) {
            let point = facing.rotate_point(point2(point.0, point.1));
            if blocked(point) { continue; }
            let (clearance_left, clearance_right) = centering(point, facing);
            min_clearance_left = clearance_left.min(min_clearance_left);
            min_clearance_right = clearance_right.min(min_clearance_right);
        }

        let (current_clearance_left, current_clearance_right) = centering(point2(0, 0), facing);
        let (current_clearance_forward, current_clearance_backward) = centering(point2(0, 0), facing.rotated(Egocentric::Left));

        if true && // current_clearance_left + current_clearance_right > current_clearance_forward + current_clearance_backward && 
            current_clearance_right > min_clearance_right {  // is there a door here?
            // are we in the center of the door?
            let rhs_direction = facing.rotated(Egocentric::Right);
            println!("we're in a door if we go: {:?}", rhs_direction);

            let mut rhs_min_clearance_left = self.distance;
            let mut rhs_min_clearance_right = self.distance;
            for point in Bresenham::new((0, 0), (0, -(current_clearance_right+ 1))) {
                let point = rhs_direction.rotate_point(point2(point.0, point.1));
                if blocked(point) { continue; }
                let (rhs_clearance_left, rhs_clearance_right) = centering(point, rhs_direction);
                rhs_min_clearance_left = rhs_clearance_left.min(rhs_min_clearance_left);
                rhs_min_clearance_right = rhs_clearance_right.min(rhs_min_clearance_right);
            }

            println!("centering? {:?} vs {:?}", rhs_min_clearance_left, rhs_min_clearance_right);
            if rhs_min_clearance_left == rhs_min_clearance_right {
                facing = rhs_direction;
            }
        }

        let mut min_clearance_left = self.distance;
        let mut min_clearance_right = self.distance;
        for point in Bresenham::new((0, self.distance + 1), (0, -(self.distance + 1))) {
            let point = facing.rotate_point(point2(point.0, point.1));
            if blocked(point) { continue; }
            let (clearance_left, clearance_right) = centering(point, facing);
            min_clearance_left = clearance_left.min(min_clearance_left);
            min_clearance_right = clearance_right.min(min_clearance_right);
        }

        self.internal_facing = facing;

        let offset_towards_center = 
            if min_clearance_left > min_clearance_right {
                facing.rotated(Egocentric::Left).offset()
            } else if min_clearance_left < min_clearance_right {
                facing.rotated(Egocentric::Right).offset()
            } else {
                vec2(0, 0)
            };

        let mut new = facing.offset() + offset_towards_center;
        let mut blocked_fwd = false;
        let mut blocked_bwd = false;
        for i in 1..self.distance {
            if blocked(point2(0, 0) + self.internal_facing.offset_by(i)) {
                blocked_fwd = true;
                break
            }
        }
        for i in 1..self.distance {
            if blocked(point2(0, 0) + self.internal_facing.offset_by(-i)) {
                blocked_bwd = true;
                break
            }
        }

        if blocked_fwd && blocked_bwd {
            println!("blocked forward and backwards: {:?}", new);
            /*
            let (current_clearance_forward, current_clearance_backward) = centering(point2(0, 0), facing.rotated(Egocentric::Left));
            if current_clearance_forward > current_clearance_backward {
                new += self.internal_facing.offset_by(1);
                println!("more clearance forward");
            } else if current_clearance_backward > current_clearance_forward {
                new += self.internal_facing.offset_by(-1);
                println!("more clearance backward");
            } else {

            }
            println!("will step: {:?}", new);
            */
        } else if blocked_fwd {
            println!("turning");
            self.internal_facing = facing.left();
            return
        }

        step(new);
    /*
        if !bresenblocked(old_look_fwd) && bresenblocked(new_look_fwd) {
            step(vec2(0, -1));
            turn(Egocentric::Left);
            return;
        }

        let look_bwd = vec2(self.distance + 1, self.distance + 1);
        if bresenblocked(old + look_bwd) && !bresenblocked(new + look_bwd) {
            turn(Egocentric::Right);
            step(vec2(0, -1));
            return;
        }

        if blocked(new) {
            turn(Egocentric::Left);
            return;
        }
        step(vec2(0, -1));
        */
    }
}