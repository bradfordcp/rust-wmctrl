use std::fmt;

pub struct Transformation {
    pub gravity: u16,
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

impl Transformation {
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Transformation {
        Transformation { gravity: 0, x, y, width, height }
    }

    pub fn set_gravity(&mut self, gravity: u16) {
        self.gravity = gravity;
    }
}

impl fmt::Display for Transformation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{},{},{},{}", self.gravity, self.x, self.y, self.width, self.height)
    }
}