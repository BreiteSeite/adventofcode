use std::fs::read_to_string;

use submarine::sonar::{self, SonarReadings};

pub mod submarine;

fn main() {
    let sonar_readings = SonarReadings(
        read_to_string("input/day1_sonar_sweep")
            .unwrap()
            .lines()
            .map(|r| r.parse::<i32>().unwrap())
            .collect(),
    );

    day1_puzzle1(sonar_readings.clone());
    day1_puzzle2(sonar_readings.clone());
}

fn day1_puzzle1(sonar_readings: SonarReadings) {
    println!("depth increases: {}", sonar_readings.depth_increases());
}

fn day1_puzzle2(sonar_readings: SonarReadings) {
    println!(
        "depth increases (3-measurement window): {}",
        sonar_readings.depth_increases_windowed(3)
    );
}
