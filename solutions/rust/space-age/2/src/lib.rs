// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration(f64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self(s as f64)
    }
}

pub trait Planet {
    const FACTOR: f64 = 1.0;
    fn years_during(d: &Duration) -> f64 {
        d.0 / Self::FACTOR / 31_557_600.0
    }
}
// first attempt
// pub struct Mercury;
// pub struct Venus;
// pub struct Earth;
// pub struct Mars;
// pub struct Jupiter;
// pub struct Saturn;
// pub struct Uranus;
// pub struct Neptune;
//
// impl Planet for Mercury {
//     const FACTOR: f64 = 0.2408467;
// }
// impl Planet for Venus {
//     const FACTOR: f64 = 0.61519726;
// }
// impl Planet for Earth {}
// impl Planet for Mars {
//     const FACTOR: f64 = 1.8808158;
// }
// impl Planet for Jupiter {
//     const FACTOR: f64 = 11.862615;
// }
// impl Planet for Saturn {
//     const FACTOR: f64 = 29.447498;
// }
// impl Planet for Uranus {
//     const FACTOR: f64 = 84.016846;
// }
// impl Planet for Neptune {
//     const FACTOR: f64 = 164.79132;
// }

// second attempt using macro
macro_rules! planets {
    ($($name:ident => $orbital_period:expr),* $(,)?) => {
        $(
            pub struct $name;
            impl Planet for $name {
                const FACTOR: f64 = $orbital_period;

            }
        )*
    };
}

planets!(
    Mercury => 0.2408467,
    Venus => 0.61519726,
    Earth => 1.0,
    Mars => 1.8808158,
    Jupiter => 11.862615,
    Saturn => 29.447498,
    Uranus => 84.016846,
    Neptune => 164.79132
);