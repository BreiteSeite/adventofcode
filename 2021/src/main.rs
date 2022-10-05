use std::fs::read_to_string;

use submarine::sonar::SonarReadings;

pub mod submarine;

fn main() {
    day1_puzzle1();
}

fn day1_puzzle1() {
    let sonar_readings = read_to_string("input/day1_sonar_sweep")
        .unwrap()
        .lines()
        .map(|r| r.parse::<i32>().unwrap())
        .collect();
    println!(
        "depth increases: {}",
        SonarReadings(sonar_readings).depth_increases()
    );
}
