type SonarReading = i32;

pub struct SonarReadings(pub Vec<SonarReading>);

impl SonarReadings {
    pub fn depth_increases(&self) -> i32 {
        self.0.iter().zip(self.0.iter().skip(1)).fold(
            0,
            |sum, (current, next)| {
                if next > current {
                    sum + 1
                } else {
                    sum
                }
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::submarine::sonar::SonarReadings;

    #[test]
    fn multiple_readings_continously_increasing() {
        let readings = SonarReadings(vec![1, 2, 3]);
        assert_eq!(readings.depth_increases(), 2);
    }

    #[test]
    fn no_readings() {
        let readings = SonarReadings(vec![]);
        assert_eq!(readings.depth_increases(), 0);
    }

    #[test]
    fn multiple_readings_mixed_order() {
        let readings = SonarReadings(vec![0, 5, 10, 5, 10, 11, 10]);
        assert_eq!(readings.depth_increases(), 4);
    }

    #[test]
    fn only_decreasing() {
        let readings = SonarReadings(vec![10, 9, 8, 7, 6]);
        assert_eq!(readings.depth_increases(), 0);
    }
}
