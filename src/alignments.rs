#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum VerticalAlignment {
    Top,
    Center,
    Bottom,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum HorizontalAlignment {
    Left,
    Center,
    Right,
    Justify,
}

#[derive(Clone, Debug)]
pub struct Alignment {
    pub horizontal_align: HorizontalAlignment,
    pub vertical_align: VerticalAlignment,
}
#[allow(dead_code)]
impl Alignment {
    pub fn center() -> Self {
        Self {
            horizontal_align: HorizontalAlignment::Center,
            vertical_align: VerticalAlignment::Center,
        }
    }

    pub fn top_left() -> Self {
        Self {
            horizontal_align: HorizontalAlignment::Left,
            vertical_align: VerticalAlignment::Top,
        }
    }

    pub fn top_right() -> Self {
        Self {
            horizontal_align: HorizontalAlignment::Right,
            vertical_align: VerticalAlignment::Top,
        }
    }

    pub fn bottom_left() -> Self {
        Self {
            horizontal_align: HorizontalAlignment::Left,
            vertical_align: VerticalAlignment::Bottom,
        }
    } 

    pub fn bottom_right() -> Self {
        Self {
            horizontal_align: HorizontalAlignment::Right,
            vertical_align: VerticalAlignment::Bottom,
        }
    } 
}
