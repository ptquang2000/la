use std::fmt;

#[derive(Clone, Copy, Debug)]
struct Vec2 {
    x: f64,
    y: f64,
}

impl std::ops::Add for Vec2 {
    type Output = Vec2;
    fn add(self, other: Vec2) -> Self::Output {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Mul<Vec2> for f64 {
    type Output = Vec2;
    fn mul(self, v: Vec2) -> Self::Output {
        Vec2 {
            x: self * v.x,
            y: self * v.y,
        }
    }
}

impl std::fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let v = Vec2 { x: 4., y: 2. };
    let w = Vec2 { x: -1., y: 2. };

    println!("v = {}, w = {}", v, w);
    println!("v + w = {}", v + w);
    println!("-v = {}", -1. * v);
}
