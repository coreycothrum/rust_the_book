#[derive(Debug, PartialEq)]
enum Temperature {
    Celsius(f64),
    Fahrenheit(f64),
}

impl Temperature {
    fn to_celsius(self) -> Self {
        let c = match self {
            Self::Celsius(c) => c,
            Self::Fahrenheit(f) => ((f - 32.0) * 5.0) / 9.0,
        };
        return Self::Celsius(c);
    }

    fn to_fahrenheit(self) -> Self {
        let f = match self {
            Self::Celsius(c) => (c * (9.0 / 5.0)) + 32.0,
            Self::Fahrenheit(f) => f,
        };
        return Self::Fahrenheit(f);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CELSIUS_VALS: [f64; 1] = [00.0];
    const FAHRENHEIT_VALS: [f64; 1] = [32.0];

    #[test]
    fn celsius_to_whatever() {
        for (i, c) in CELSIUS_VALS.iter().enumerate() {
            assert_eq!(
                Temperature::Celsius(*c).to_fahrenheit(),
                Temperature::Fahrenheit(FAHRENHEIT_VALS[i])
            )
        }
    }

    #[test]
    fn fahrenheit_to_whatever() {
        for (i, f) in FAHRENHEIT_VALS.iter().enumerate() {
            assert_eq!(
                Temperature::Fahrenheit(*f).to_celsius(),
                Temperature::Celsius(CELSIUS_VALS[i])
            )
        }
    }
}
