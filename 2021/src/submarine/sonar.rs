type SonarReading = i32;

pub struct SonarReadings {
    pub readings: Vec<SonarReading>,
}

impl SonarReadings {
    pub fn depth_increases(&self) -> i32 {
        self.readings
            .iter()
            .zip(self.readings.iter().skip(1))
            .fold(
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

    pub fn depth_increases_windowed(&self, window_size: usize) -> i32 {
        self.readings
            .windows(window_size)
            .zip(self.readings.windows(window_size).skip(1))
            .fold(0, |sum, (current, next)| {
                if next.iter().sum::<i32>() > current.iter().sum::<i32>() {
                    sum + 1
                } else {
                    sum
                }
            })
    }
}

#[cfg(test)]
mod tests {
    use crate::submarine::sonar::SonarReadings;

    #[test]
    fn multiple_readings_continously_increasing() {
        let readings = SonarReadings {
            readings: vec![1, 2, 3],
        };
        assert_eq!(readings.depth_increases(), 2);
    }

    #[test]
    fn no_readings() {
        let readings = SonarReadings { readings: vec![] };
        assert_eq!(readings.depth_increases(), 0);
    }

    #[test]
    fn multiple_readings_mixed_order() {
        let readings = SonarReadings {
            readings: vec![0, 5, 10, 5, 10, 11, 10],
        };
        assert_eq!(readings.depth_increases(), 4);
    }

    #[test]
    fn only_decreasing() {
        let readings = SonarReadings {
            readings: vec![10, 9, 8, 7, 6],
        };
        assert_eq!(readings.depth_increases(), 0);
    }

    #[test]
    fn multiple_readings_continously_increasing_windows() {
        let readings = SonarReadings {
            readings: vec![1, 2, 3, 4],
        };
        assert_eq!(readings.depth_increases_windowed(3), 1);
    }

    #[test]
    fn only_decreasing_windowed() {
        let readings = SonarReadings {
            readings: vec![10, 9, 8, 7, 6],
        };
        assert_eq!(readings.depth_increases_windowed(3), 0);
    }

    #[test]
    fn multiple_readings_mixed_order_windowed() {
        /*
        0 5 10 = 15 (no previous sum)
        5 10 5 = 20 (increased)
        10 5 10 = 25 (increased)
        5 10 11 = 26 (increased)
        10 11 10 = 31 (increased)
         */
        let readings = SonarReadings {
            readings: vec![0, 5, 10, 5, 10, 11, 10],
        };
        assert_eq!(readings.depth_increases_windowed(3), 4);
    }

    #[test]
    fn no_readings_windowed() {
        let readings = SonarReadings { readings: vec![] };
        assert_eq!(readings.depth_increases_windowed(3), 0);
    }
}
