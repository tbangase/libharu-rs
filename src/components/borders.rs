use crate::colors::Cmyk;

#[derive(Clone, Debug)]
pub struct Border {
    pub enable: bool,
    pub width: f32,
    pub color: Cmyk,
}

impl Border {
    pub fn no_border() -> Self {
        Self {
            enable: false,
            width: 0.0,
            color: Cmyk::white(),
        }
    }
}



#[derive(Clone, Debug)]
pub struct Outline {
    pub top: Border,
    pub bottom: Border,
    pub left: Border,
    pub right: Border,
}

#[allow(dead_code)]
impl Outline {
    pub fn new(enable: bool, width: f32, color: Cmyk) -> Self {
        let border = Border {
            enable,
            width,
            color,
        };
        return Self {
            top: border.clone(),
            bottom: border.clone(),
            left: border.clone(),
            right: border.clone(),
        };
    }

    pub fn from_border(border: Border) -> Self {
        return Self {
            top: border.clone(),
            bottom: border.clone(),
            left: border.clone(),
            right: border.clone(),
        };
    }

    pub fn set_top(&mut self, border: Border) {
        self.top = border;
    }

    pub fn set_bottom(&mut self, border: Border) {
        self.bottom = border;
    }

    pub fn set_left(&mut self, border: Border) {
        self.left = border;
    }

    pub fn set_right(&mut self, border: Border) {
        self.right = border;
    }
}
