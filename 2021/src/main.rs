use std::fs::read_to_string;

use submarine::sonar::SonarReadings;

pub mod submarine;

fn main() {
    let sonar_readings = SonarReadings(
        read_to_string("input/day1_sonar_sweep")
            .unwrap()
            .lines()
            .map(|r| r.parse::<i32>().unwrap())
            .collect(),
    );

    day1_puzzle1(&sonar_readings);
    day1_puzzle2(&sonar_readings);
}

fn day1_puzzle1(sonar_readings: &SonarReadings) {
    println!("depth increases: {}", sonar_readings.depth_increases());
}

fn day1_puzzle2(sonar_readings: &SonarReadings) {
    println!(
        "depth increases (3-measurement window): {}",
        sonar_readings.depth_increases_windowed(3)
    );
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use crate::submarine::sonar::SonarReadings;

    #[test]
    fn day1_puzzle1_solution() {
        let sonar_readings = SonarReadings(
            read_to_string("input/day1_sonar_sweep")
                .unwrap()
                .lines()
                .map(|r| r.parse::<i32>().unwrap())
                .collect(),
        );

        assert_eq!(sonar_readings.depth_increases(), 1215);
    }

    #[test]
    fn day1_puzzle2_solution() {
        let sonar_readings = SonarReadings(
            read_to_string("input/day1_sonar_sweep")
                .unwrap()
                .lines()
                .map(|r| r.parse::<i32>().unwrap())
                .collect(),
        );

        assert_eq!(sonar_readings.depth_increases_windowed(3), 1150);
    }
}
