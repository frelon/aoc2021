use std::ops::Add;
use std::ops::Sub;

#[derive(Debug,Clone,Copy,Eq,PartialEq)]
struct Pos(i32,i32);

impl Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Pos) -> Pos {
        Pos(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Pos {
    type Output = Pos;

    fn sub(self, rhs: Pos) -> Pos {
        Pos(self.0 - rhs.0, self.1 - rhs.1)
    }
}

#[derive(Debug,Clone,Copy)]
struct Rect(Pos,Pos);

fn main() {
    // let max_y = reach(Pos(0,0), Pos(6,0), Rect(Pos(20,-10),Pos(30,-5)), -100); 
    // println!("answer1: {:?}", max_y);

    // let steps = brute(Rect(Pos(20,-10),Pos(30,-5)));
    let steps = brute(Rect(Pos(288,-96),Pos(330,-50)));

    let max_y = steps.iter().map(|(_,y)| y).max().unwrap();

    println!("answer1: {}", max_y);
    println!("answer2: {}", steps.len());
}

fn brute(target:Rect) -> Vec<(Pos,i32)> {
    let max = std::cmp::max(target.0.0, target.1.0) + 1;
    let mut result = vec![];
    for x in 0..=max {
        for y in -max..=max {
            match reach(Pos(0,0),Pos(x,y),target, -100) {
                Some(max_y) => result.push((Pos(x,y), max_y)),
                None => continue,
            }
        }
    }
    result
}

fn reach(probe:Pos,velocity:Pos,target:Rect,max_y:i32) -> Option<i32> {
    // println!("Pos: {:?}, vel: {:?}", probe, velocity);

    if has_reached(probe, target) {
        return Some(max_y);
    }

    if has_missed(probe, velocity, target) {
        return None;
    }

    reach(probe+velocity, drag(velocity), target, std::cmp::max(max_y,probe.1))
}

fn drag(velocity:Pos) -> Pos {
    let mut change = Pos(0, -1);

    if velocity.0 < 0 {
        change.0 = 1;
    }

    if velocity.0 > 0 {
        change.0 = -1;
    }

    return velocity + change
}

fn has_reached(pos:Pos,target:Rect) -> bool {
    pos.0 >= target.0.0 && pos.0 <= target.1.0 &&
    pos.1 >= target.0.1 && pos.1 <= target.1.1
}

fn has_missed(pos:Pos,vel:Pos,target:Rect) -> bool {
    pos.1 < target.0.1 || (vel.0 == 0 && (pos.0 < target.0.0 && pos.0 > target.1.0) )
}
