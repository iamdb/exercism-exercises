const EARTH_YEAR_SECONDS: f64 = 31557600.;

#[derive(Debug)]
pub struct Duration {
    seconds: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { seconds: s as f64 }
    }
}

macro_rules! planet {
    ($planet:ident, $orbital_modifier: expr) => {
        pub struct $planet {}

        impl $planet {
            pub fn years_during(d: &Duration) -> f64 {
                d.seconds / (EARTH_YEAR_SECONDS * $orbital_modifier)
            }
        }
    };
}

planet!(Mercury, 0.2408467);
planet!(Venus, 0.61519726);
planet!(Earth, 1.);
planet!(Mars, 1.8808158);
planet!(Jupiter, 11.862615);
planet!(Saturn, 29.447498);
planet!(Uranus, 84.016846);
planet!(Neptune, 164.79132);
