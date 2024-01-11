mod measurement;

use std::{
    fs::File,
    io::{BufRead, BufReader},
    process::exit,
};

use measurement::Stations;

/// The file path of the measurement file.
const MEASUREMENT_FILE: &str = "./measurements.txt";

/// Reads the measurement file and returns a `BufRead` object.
/// Returns `None` if the file cannot be opened.
fn read_measurement_file() -> Option<impl BufRead> {
    match File::open(MEASUREMENT_FILE) {
        Ok(file) => Some(BufReader::new(file)),
        Err(_) => None,
    }
}

/// Parses a line of measurement data and returns a tuple of station name and measurement value.
/// Returns `None` if the line is in the wrong format.
fn parse_line(line: &str) -> Option<(String, f32)> {
    line.split_once(";").and_then(|(station, measurement_str)| {
        measurement_str
            .parse::<f32>()
            .ok()
            .map(|measurement| (station.to_string(), measurement))
    })
}

fn main() {
    let mut stations = Stations::new();

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

        stations.add_measurement(&station, measurement);
    }

    println!("{}", stations);
}

#[cfg(test)]
mod tests {
    use crate::parse_line;

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
}
