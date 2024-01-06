// The text file contains temperature values for a range of weather stations.
//Each row is one measurement in the format <string: station name>;<double: measurement>,
//with the measurement value having exactly one fractional digit.
// The following shows ten rows as an example:

// Hamburg;12.0
// Bulawayo;8.9
// Palembang;38.8
// St. John's;15.2
// Cracow;12.6
// Bridgetown;26.9
// Istanbul;6.2
// Roseau;34.4
// Conakry;31.2
// Istanbul;23.0

// The task is to write a Rust program which reads the file, calculates the min, mean, and max temperature value per weather station,
// and emits the results on stdout like this (i.e. sorted alphabetically by station name, and the result values per station in the format <min>/<mean>/<max>, rounded to one fractional digit):

// {Abha=-23.0/18.0/59.2, Abidjan=-16.2/26.0/67.3, Abéché=-10.0/29.4/69.0, Accra=-10.1/26.4/66.4, Addis Ababa=-23.7/16.0/67.0, Adelaide=-27.8/17.3/58.5, ...}

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    process::exit,
};

const MEASUREMENT_FILE: &str = "./measurements.txt";

fn calculate_average(
    station: &str,
    measurement: f32,
    measurements: &mut HashMap<String, Measurement>,
) {
    println!("Stations is {}, measurement is {}", station, measurement);

    let station = measurements
        .entry(station.to_string())
        .or_insert(Measurement::default());

    station.update(measurement);

    // stations.
}

fn read_measurement_file() -> Option<impl BufRead> {
    match File::open(MEASUREMENT_FILE) {
        Ok(file) => Some(BufReader::new(file)),
        Err(_) => None,
    }
}

fn parse_line(line: &str) -> Option<(String, f32)> {
    line.split_once(";").and_then(|(station, measurement_str)| {
        measurement_str
            .parse::<f32>()
            .ok()
            .map(|measurement| (station.to_string(), measurement))
    })
}

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

impl Measurement {
    pub fn update(&mut self, measurement: f32) {
        self.min = self.min.min(measurement);
        self.max = self.max.max(measurement);
        self.count = self.count + 1;
        self.sum = self.sum + measurement;
    }
}

fn main() {
    let mut map: HashMap<String, Measurement> = HashMap::new();

    let buffer = read_measurement_file().unwrap_or_else(|| {
        println!("Cannot open {}", MEASUREMENT_FILE);
        exit(1);
    });

    for result in buffer.lines() {
        let line = result.unwrap_or_else(|err| {
            println!(
                "Cannot properly read line from {}. Error: {}",
                MEASUREMENT_FILE, err
            );
            exit(1);
        });

        let (station, measurement) = parse_line(&line).unwrap_or_else(|| {
            println!("Line in the wrong format, cannot split by ;");
            exit(1);
        });

        calculate_average(&station, measurement, &mut map);
    }
}

#[cfg(test)]
mod tests {
    use crate::{parse_line, Measurement};

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("Bulawayo;8.9"),
            Some(("Bulawayo".to_string(), 8.9))
        );
        assert_eq!(
            parse_line("St. John's;15.2"),
            Some(("St. John's".to_string(), 15.2))
        );
        assert_eq!(
            parse_line("Istanbul;6.2"),
            Some(("Istanbul".to_string(), 6.2))
        );
        assert_eq!(parse_line("Invalid Line"), None);
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
}
