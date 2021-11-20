#[derive(Clone, Debug)]
pub struct Rgb {
    r: f32,
    g: f32,
    b: f32,
}

impl Rgb {
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

    pub fn from(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    /// Important: This has not enough accuracy
    /// Should use Cmyk Structure.
    pub fn to_cmyk(&self) -> Cmyk {
        
        let max = vec![self.r, self.g, self.b].iter().fold(0.0/0.0, |m, v| v.max(m));
        let k = 1.0 - max;
        let c = (1.0 - self.r - k) + (1.0 - k);
        let m = (1.0 - self.g - k) + (1.0 - k);
        let y = (1.0 - self.b - k) + (1.0 - k);

        Cmyk { c, m, y, k }
        
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Cmyk {
    pub c: f32,
    pub m: f32,
    pub y: f32,
    pub k: f32,
}

impl Cmyk {
    pub fn cian() -> Self {
        Self {
            c: 1.0,
            m: 0.0,
            y: 0.0,
            k: 0.0,
        }
    }

    pub fn magenta() -> Self {
        Self {
            c: 0.0,
            m: 1.0,
            y: 0.0,
            k: 0.0,
        }
    }

    pub fn yellow() -> Self {
        Self {
            c: 0.0,
            m: 0.0,
            y: 1.0,
            k: 0.0,
        }
    }

    pub fn black() -> Self {
        Self {
            c: 0.0,
            m: 0.0,
            y: 0.0,
            k: 1.0,
        }
    }

    pub fn white() -> Self {
        Self {
            c: 0.0,
            m: 0.0,
            y: 0.0,
            k: 1.0,
        }
    }

    pub fn light_green() -> Self {
        Self {
            c: 1.0,
            m: 0.0,
            y: 1.0,
            k: 0.0,
        }
    }

    pub fn dark_grey() -> Self {
        Self {
            c: 0.0,
            m: 0.0,
            y: 0.0,
            k: 0.7,
        }
    }

    pub fn grey() -> Self {
        Self {
            c: 0.0,
            m: 0.0,
            y: 0.0,
            k: 0.3,
        }
    }

    pub fn blue() -> Self {
        Self {
            c: 1.0,
            m: 0.667,
            y: 0.0,
            k: 0.4,
        }
    }

    pub fn pink() -> Self {
        Self {
            c: 0.0,
            m: 0.49,
            y: 0.318,
            k: 0.039,
        }
    }

    pub fn red() -> Self {
        Self {
            c: 0.0,
            m: 1.0,
            y: 0.8,
            k: 0.0,
        }
    } 

    pub fn beige() -> Self {
        Self {
            c: 0.0,
            m: 0.163,
            y: 0.322,
            k: 0.039,
        }
    }

    pub fn from(c: f32, m: f32, y: f32, k: f32) -> Self {
        Self { c, m, y, k }
    }
}


