use rand_distr::{Distribution, Normal};

/// Represents a weather station with an ID and a mean temperature.
#[derive(Debug)]
pub struct WeatherStation {
    id: String,
    mean_temperature: f32,
}

impl WeatherStation {
    /// Creates a new weather station with the given ID and mean temperature.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the weather station.
    /// * `mean_temperature` - The mean temperature of the weather station.
    ///
    /// # Example
    ///
    /// ```
    /// let station = WeatherStation::new("Milan".to_string(), 25.0);
    /// ```
    pub fn new(id: String, mean_temperature: f32) -> Self {
        Self {
            id,
            mean_temperature,
        }
    }

    /// Returns the ID of the weather station.
    ///
    /// # Returns
    ///
    /// The ID of the weather station.
    pub fn id(&self) -> &str {
        self.id.as_ref()
    }

    /// Generates a random measurement of temperature based on the mean temperature of the weather station.
    ///
    /// # Returns
    ///
    /// The generated measurement of temperature.
    ///
    /// # Example
    ///
    /// ```
    /// let station = WeatherStation::new("Milan".to_string(), 25.0);
    /// let measurement = station.measurement();
    /// ```
    pub fn measurement(&self) -> f32 {
        let normal = Normal::new(self.mean_temperature, 10.0).unwrap();
        let mut rng = rand::thread_rng();

        let random_temperature = normal.sample(&mut rng);

        (random_temperature * 10.0).round() / 10.0
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;

    use super::*;

    #[test]
    fn test_measurement() {
        let station = WeatherStation::new("Milan".to_string(), 25.0);
        let mut measurements = Vec::new();

        // Generate 1000 measurements and store them in the `measurements` vector
        for _ in 0..1000 {
            measurements.push(station.measurement());
        }

        // Calculate the mean temperature
        let mean: f32 = measurements.iter().sum::<f32>() / measurements.len() as f32;

        // Calculate the variance
        let variance = measurements
            .iter()
            .map(|&measurement| (measurement - mean).powi(2))
            .sum::<f32>()
            / measurements.len() as f32;

        // Calculate the standard deviation
        let std_dev = variance.sqrt();

        // The computed mean temperature is within 1 degree of the station mean temperature
        assert_abs_diff_eq!(mean, station.mean_temperature, epsilon = 1.0);

        // The computed standard deviation is within 1 degree of 10.0
        assert_abs_diff_eq!(std_dev, 10.0, epsilon = 1.0);
    }
}
