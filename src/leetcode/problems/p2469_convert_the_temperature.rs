fn convert_to_kelvin(celsius: f64) -> f64 {
    celsius + 273.15
}

fn convert_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 1.8 + 32.0
}

pub fn convert_temperature(celsius: f64) -> Vec<f64> {
    vec![convert_to_kelvin(celsius), convert_to_fahrenheit(celsius)]
}

#[cfg(test)]
mod test {
    use super::convert_temperature;

    #[test]
    fn cases() {
        assert_eq!(convert_temperature(36.50), vec![309.65000, 97.70000]);
        assert_eq!(convert_temperature(122.11), vec![395.26000, 251.79800]);
    }
}
