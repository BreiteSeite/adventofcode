use std::fs::read_to_string;

use crate::submarine::sonar::SonarReadings;

pub fn day1() {
    println!("Day 1 | Puzzle 1: {}", day1_puzzle1());
    println!("Day 1 | Puzzle 2: {}", day1_puzzle2());
}

pub fn day1_puzzle1() -> i32 {
    let sonar_readings = parse_sonar_sweep_file();

    sonar_readings.depth_increases()
}

pub fn day1_puzzle2() -> i32 {
    let sonar_readings = parse_sonar_sweep_file();

    sonar_readings.depth_increases_windowed(3)
}

fn parse_sonar_sweep_file() -> SonarReadings {
    let sonar_readings = SonarReadings {
        readings: read_to_string("input/day1_sonar_sweep")
            .unwrap()
            .lines()
            .map(|r| r.parse::<i32>().unwrap())
            .collect(),
    };
    sonar_readings
}

#[cfg(test)]
mod tests {
    use crate::day1::{day1_puzzle1, day1_puzzle2};

    #[test]
    fn day1_puzzle1_solution() {
        assert_eq!(day1_puzzle1(), 1215);
    }

    #[test]
    fn day1_puzzle2_solution() {
        assert_eq!(day1_puzzle2(), 1150);
    }
}
