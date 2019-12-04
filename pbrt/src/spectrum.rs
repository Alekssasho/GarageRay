use std::ops::AddAssign;
use std::ops::Div;
use std::ops::Mul;

pub struct Spectrum {}

impl Spectrum {
    pub fn new() -> Spectrum {
        Spectrum {}
    }

    pub fn is_black(&self) -> bool {
        false
    }
}

impl AddAssign for Spectrum {
    fn add_assign(&mut self, other: Spectrum) {
        *self = Spectrum {};
    }
}

impl Mul for Spectrum {
    type Output = Self;

    fn mul(self, rhs: Self) -> Spectrum {
        self
    }
}

impl Mul<f32> for Spectrum {
    type Output = Self;
    fn mul(self, rhs: f32) -> Spectrum {
        self
    }
}

impl Div<f32> for Spectrum {
    type Output = Self;

    fn div(self, rhs: f32) -> Spectrum {
        self
    }
}
