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
    fs::File,
    io::{BufRead, BufReader},
    process::exit,
};

const MEASUREMENT_FILE: &str = "./measurements.txt";

fn calculate_average(station: &str, measurement: f32) {
    println!("Stations is {}, measurement is {}", station, measurement);
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

fn main() {
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

        calculate_average(&station, measurement);
    }
}

#[cfg(test)]
mod tests {
    use crate::parse_line;

    #[test]
    fn test_parse_line() {
        let line = "Milan;13.0";

        let (station, measurement) = parse_line(line).unwrap();

        assert_eq!(station, "Milan");
        assert_eq!(measurement, 13.0);
    }

    #[test]
    fn test_parse_line_returns_none() {
        let line = "Milan 13.0";

        let actual = parse_line(line);

        assert!(actual.is_none());
    }
}
