use std::{collections::HashMap, fmt::Display};

#[derive(Debug)]
pub struct Stations {
    data: HashMap<String, Measurement>,
}

impl Display for Stations {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{{")?;

        let sorted: Vec<(String, Measurement)> = self.to_sorted_vector();

        for (station, measurement) in sorted {
            write!(formatter, "{}={},", station, measurement)?;
        }

        write!(formatter, "}}")?;
        Ok(())
    }
}

impl Stations {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn add_measurement(&mut self, station: &str, measurement: f32) {
        let current = &mut self
            .data
            .entry(station.to_string())
            .or_insert(Measurement::default());

        current.update(measurement);
    }

    fn to_sorted_vector(&self) -> Vec<(String, Measurement)> {
        let mut vec: Vec<(String, Measurement)> = self.data.clone().into_iter().collect();
        vec.sort_by(|(a, _), (b, _)| a.cmp(b));

        vec
    }
}

#[derive(Debug, Clone)]
pub struct Measurement {
    min: f32,
    max: f32,
    sum: f32,
    count: i32,
}

impl Default for Measurement {
    fn default() -> Self {
        Self {
            min: f32::MAX,
            max: f32::MIN,
            sum: 0.0,
            count: 0,
        }
    }
}

impl Display for Measurement {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "{:.1}/{:.1}/{:.1}",
            self.min,
            self.mean(),
            self.max
        )
    }
}

impl Measurement {
    pub fn update(&mut self, measurement: f32) {
        self.min = self.min.min(measurement);
        self.max = self.max.max(measurement);
        self.count = self.count + 1;
        self.sum = self.sum + measurement;
    }

    pub fn mean(&self) -> f32 {
        self.sum / self.count as f32
    }
}

#[cfg(test)]
mod tests {
    use super::{Measurement, Stations};

    #[test]
    fn test_add_measurement() {
        let mut stations = Stations::new();
        stations.add_measurement("Milan", 10.0);
        stations.add_measurement("Milan", 20.0);
        stations.add_measurement("Rome", 15.0);

        assert_eq!(stations.data.len(), 2);

        let milan_measurement = stations.data.get("Milan").unwrap();
        assert_eq!(milan_measurement.min, 10.0);
        assert_eq!(milan_measurement.max, 20.0);
        assert_eq!(milan_measurement.mean(), 15.0);

        let rome_measurement = stations.data.get("Rome").unwrap();
        assert_eq!(rome_measurement.min, 15.0);
        assert_eq!(rome_measurement.max, 15.0);
        assert_eq!(rome_measurement.mean(), 15.0);
    }

    #[test]
    fn test_display_stations() {
        let mut stations = Stations::new();
        stations.add_measurement("Milano", 10.0);
        stations.add_measurement("Roma", 20.0);
        stations.add_measurement("Ancona", 25.0);

        let expected_output = "{Ancona=25.0/25.0/25.0,Milano=10.0/10.0/10.0,Roma=20.0/20.0/20.0,}";

        assert_eq!(stations.to_string(), expected_output);
    }

    #[test]
    fn test_update() {
        let mut measurement = Measurement::default();
        measurement.update(10.0);
        assert_eq!(measurement.min, 10.0);
        assert_eq!(measurement.max, 10.0);
        assert_eq!(measurement.count, 1);
        assert_eq!(measurement.sum, 10.0);

        measurement.update(5.0);
        assert_eq!(measurement.min, 5.0);
        assert_eq!(measurement.max, 10.0);
        assert_eq!(measurement.count, 2);
        assert_eq!(measurement.sum, 15.0);

        measurement.update(15.0);
        assert_eq!(measurement.min, 5.0);
        assert_eq!(measurement.max, 15.0);
        assert_eq!(measurement.count, 3);
        assert_eq!(measurement.sum, 30.0);
    }

    #[test]
    fn test_mean() {
        let measurement = Measurement {
            min: 10.0,
            max: 25.0,
            count: 7,
            sum: 15.0,
        };

        let actual = measurement.mean();

        assert_eq!(actual, measurement.sum / measurement.count as f32);
    }
}
