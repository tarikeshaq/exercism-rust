// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration(f64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s as f64 / 31_557_600.0)
    }
}

pub trait Planet {
    const PERIOD: f64;
    fn years_during(d: &Duration) -> f64 {
        d.0 / Self::PERIOD
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    const PERIOD: f64 = 0.240_846_7;
}
impl Planet for Venus {
    const PERIOD: f64 = 0.615_197_26;
}
impl Planet for Earth {
    const PERIOD: f64 = 1.0;
}
impl Planet for Mars {
    const PERIOD: f64 = 1.880_815_8;
}
impl Planet for Jupiter {
    const PERIOD: f64 = 11.862_615;
}
impl Planet for Saturn {
    const PERIOD: f64 = 29.447_498;
}
impl Planet for Uranus {
    const PERIOD: f64 = 84.016_846;
}
impl Planet for Neptune {
    const PERIOD: f64 = 164.791_32;
}
