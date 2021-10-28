pub struct Position {
    pub x: i32,
    pub y: i32
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Position{
            x: x,
            y: y
        }
    }

    pub fn is_same(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
