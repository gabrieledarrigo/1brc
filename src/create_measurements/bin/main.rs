mod create_measurements;
mod weather_station;

use std::{env, process::exit};

use create_measurements::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: create_measurements <number of records to create>");
        exit(1);
    }

    let number_of_measurements = args[1].parse::<i32>().unwrap_or_else(|_| {
        println!("The value provided is not a valid number");
        exit(1);
    });

    create_measurements(number_of_measurements);
}
