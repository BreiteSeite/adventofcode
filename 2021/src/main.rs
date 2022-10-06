use std::fs::read_to_string;

use submarine::{control::parse_control_inputs, sonar::SonarReadings, Submarine};

pub mod submarine;

fn main() {
    day1();

    day2();
}

fn day1() {
    let sonar_readings = SonarReadings {
        readings: read_to_string("input/day1_sonar_sweep")
            .unwrap()
            .lines()
            .map(|r| r.parse::<i32>().unwrap())
            .collect(),
    };
    day1_puzzle1(&sonar_readings);
    day1_puzzle2(&sonar_readings);
}

fn day1_puzzle1(sonar_readings: &SonarReadings) {
    println!("Day 1 | Puzzle 1: {}", sonar_readings.depth_increases());
}

fn day1_puzzle2(sonar_readings: &SonarReadings) {
    println!(
        "Day 1 | Puzzle 2: {}",
        sonar_readings.depth_increases_windowed(3)
    );
}

fn day2() {
    println!("Day 2 | Puzzle 1: {}", day2_puzzle1())
}

fn day2_puzzle1() -> u32 {
    let control_inputs = parse_control_inputs("input/day2_dive");
    let mut submarine = Submarine::default();

    for input in control_inputs {
        submarine.applyControlInput(input);
    }

    submarine.position.horizontal * submarine.position.depth
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use crate::{day2_puzzle1, submarine::sonar::SonarReadings};

    #[test]
    fn day1_puzzle1_solution() {
        let sonar_readings = SonarReadings {
            readings: read_to_string("input/day1_sonar_sweep")
                .unwrap()
                .lines()
                .map(|r| r.parse::<i32>().unwrap())
                .collect(),
        };

        assert_eq!(sonar_readings.depth_increases(), 1215);
    }

    #[test]
    fn day1_puzzle2_solution() {
        let sonar_readings = SonarReadings {
            readings: read_to_string("input/day1_sonar_sweep")
                .unwrap()
                .lines()
                .map(|r| r.parse::<i32>().unwrap())
                .collect(),
        };

        assert_eq!(sonar_readings.depth_increases_windowed(3), 1150);
    }

    #[test]
    fn day2_puzzle2_solution() {
        assert_eq!(1427868, day2_puzzle1());
    }
}
