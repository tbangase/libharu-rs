#[derive(Clone)]
enum VerticalAlignment {
    Top,
    Center,
    Bottom,
}

#[derive(Clone)]
enum HorizontalAlignment {
    Left,
    Center,
    Right,
    Justify,
}

#[derive(Clone)]
struct Alignment {
    horizontal_align: HorizontalAlignment,
    vertical_align: VerticalAlignment,
}

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