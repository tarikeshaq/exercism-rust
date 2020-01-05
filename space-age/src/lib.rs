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
    fn years_during(d: &Duration) -> f64 {
        unimplemented!(
            "convert a duration ({:?}) to the number of years on this planet for that duration",
            d,
        );
    }
}

const MERCURY_PERIOD: f64 = 0.240_846_7;
const VENUS_PERIOD: f64 = 0.615_197_26;
const MARS_PERIOD: f64 = 1.880_815_8;
const JUPITER_PERIOD: f64 = 11.862_615;
const SATURN_PERIOD: f64 = 29.447_498;
const URANUS_PERIOD: f64 = 84.016_846;
const NEPTUNE_PERIOD: f64 = 164.791_32;

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn years_during(d: &Duration) -> f64 {
        d.0 / MERCURY_PERIOD
    }
}
impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        d.0 / VENUS_PERIOD
    }
}
impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        d.0
    }
}
impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        d.0 / MARS_PERIOD
    }
}
impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        d.0 / JUPITER_PERIOD
    }
}
impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        d.0 / SATURN_PERIOD
    }
}
impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        d.0 / URANUS_PERIOD
    }
}
impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        d.0 / NEPTUNE_PERIOD
    }
}
