#[derive(Clone, Debug)]
pub struct Padding {
    pub top: f32,
    pub bottom: f32,
    pub right: f32,
    pub left: f32,
}

#[allow(dead_code)]
impl Padding {
    pub fn new_all(all: f32) -> Self {
        Self {
            top: all,
            bottom: all,
            right: all,
            left: all,
        }
    }

    pub fn new_vh(vertical: f32, horizontal: f32) -> Self {
        Self {
            top: vertical,
            bottom: vertical,
            right: horizontal,
            left: horizontal,
        }
    }

    pub fn new(top: f32, bottom: f32, right: f32, left: f32) -> Self {
        Self {
            top,
            bottom,
            right,
            left,
        }
    }

    pub fn none() -> Self {
        Self {
            top: 0.0,
            bottom: 0.0,
            right: 0.0,
            left: 0.0,
        }
    }
}
