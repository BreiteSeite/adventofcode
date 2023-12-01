use std::fs::read_to_string;

struct CalibrationLine(String);

impl CalibrationLine {
    fn extract_digits(&self) -> (char, char) {
        let chars = self.0
            .chars()
            .filter(|c| c.is_ascii_digit());

        let (a, b) = chars.clone().zip(chars.rev()).next().unwrap();

        (a, b)
    }
}

pub fn part1() {
    let input = read_to_string("input/day1.txt").unwrap();

    let part1 = calc_part1(input);

    println!("Day 1 | Part 1: {}", part1);
}

fn calc_part1(input: String) -> u32 {
    input
        .lines()
        .map(|l| CalibrationLine(l.to_string()))
        .map(|cl| cl.extract_digits())
        .map(|(n1, n2)| {
            let mut string = n1.to_string();
            string.push(n2);
            string
        })
        .map(|number_as_string| number_as_string.to_string().parse::<u32>().expect("Can not parse number from string"))
        .sum()

}
#[cfg(test)]
mod tests {
    use crate::day1::{calc_part1, CalibrationLine};

    #[test]
    fn test_extract_digits() {
        assert_eq!(('1', '2'), CalibrationLine(String::from("1abc2")).extract_digits());
        assert_eq!(('7', '7'), CalibrationLine(String::from("treb7uchet")).extract_digits());
        assert_eq!(('1', '5'), CalibrationLine(String::from("a1b2c3d4e5f")).extract_digits());
    }

    #[test]
    fn test_part1() {
        let input = String::from(r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"#);

        assert_eq!(142, calc_part1(input));
    }
}