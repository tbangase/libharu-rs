
#[derive(Clone, Debug)]
pub struct Position {
    x: f32, y:f32
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x, y
        }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }
}
