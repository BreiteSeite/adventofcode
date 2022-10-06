use std::fs::read_to_string;

use crate::submarine::sonar::SonarReadings;

pub fn day1() {
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

pub fn day1_puzzle1(sonar_readings: &SonarReadings) {
    println!("Day 1 | Puzzle 1: {}", sonar_readings.depth_increases());
}

pub fn day1_puzzle2(sonar_readings: &SonarReadings) {
    println!(
        "Day 1 | Puzzle 2: {}",
        sonar_readings.depth_increases_windowed(3)
    );
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use crate::submarine::sonar::SonarReadings;

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
}
