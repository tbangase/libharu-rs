#[derive(Clone, Debug)]
struct Color {
    r: f32,
    g: f32,
    b: f32,
}

impl Color {
    pub fn red() -> Self {
        Self {
            r: 1.0,
            g: 0.0,
            b: 0.0,
        }
    }

    pub fn green() -> Self {
        Self {
            r: 0.0,
            g: 1.0,
            b: 0.0,
        }
    }

    pub fn blue() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 1.0,
        }
    }

    pub fn sky_blue() -> Self {
        Self {
            r: 0.7,
            g: 0.8,
            b: 1.0
        }
    }

    pub fn black() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }

    pub fn white() -> Self {
        Self {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        }
    }

    pub fn dark_grey() -> Self {
        Self {
            r: 0.2,
            g: 0.2,
            b: 0.2,
        }
    }

    pub fn grey() -> Self {
        Self {
            r: 0.3,
            g: 0.3,
            b: 0.3,
        }
    }

    pub fn yellow() -> Self {
        Self {
            r: 1.0,
            g: 1.0,
            b: 0.5,
        }
    }

    pub fn pink() -> Self {
        Self {
            r: 1.0,
            g: 0.7,
            b: 0.7,
        }
    }

    pub fn orange() -> Self {
        Self {
            r: 1.0,
            g: 0.6,
            b: 0.3,
        }
    } 

    pub fn light_grey() -> Self {
        Self {
            r: 0.7,
            g: 0.7,
            b: 0.7,
        }
    }

    pub fn from_Color(r: f32, g: f32, b: f32) -> Self {
        Self { r: r, g: g, b: b }
    }
}