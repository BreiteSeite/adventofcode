struct CalibrationLine(String);

impl CalibrationLine {
    fn extract_digits(&self) -> (char, char) {
        let chars = self.0
            .chars()
            .filter(|c| c.is_ascii_digit());

        let (a, b) = chars.clone().zip(chars.rev()).next().unwrap();

        (a, b)
    }

    fn extract_digits_with_words(&self) -> (char, char) {
        let replaced = self.0
            .replace("one", "one1one")
            .replace("two", "two2two")
            .replace("three", "three3three")
            .replace("four", "four4four")
            .replace("five", "five5five")
            .replace("six", "six6six")
            .replace("seven", "seven7seven")
            .replace("eight", "eight8eight")
            .replace("nine", "nine9nine")
            .replace("zero", "zero0zero");

        CalibrationLine(replaced).extract_digits()
    }
}

pub fn part1() {
    let input = include_str!("../input/day1.txt");

    let part1 = calc_part1(input);

    println!("Day 1 | Part 1: {}", part1);
}

pub fn part2() {
    let input = include_str!("../input/day1.txt");

    let part2 = calc_part2(input);

    println!("Day 1 | Part 2: {}", part2);
}

fn calc_part1(input: &str) -> u32 {
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

fn calc_part2(input: &str) -> u32 {
    input
        .lines()
        .map(|l| CalibrationLine(l.to_string()))
        .map(|cl| cl.extract_digits_with_words())
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
    use crate::day1::{calc_part1, calc_part2, CalibrationLine};

    #[test]
    fn test_extract_digits() {
        assert_eq!(('1', '2'), CalibrationLine(String::from("1abc2")).extract_digits());
        assert_eq!(('7', '7'), CalibrationLine(String::from("treb7uchet")).extract_digits());
        assert_eq!(('1', '5'), CalibrationLine(String::from("a1b2c3d4e5f")).extract_digits());
    }

    #[test]
    fn test_part1() {
        let input = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"#;

        assert_eq!(142, calc_part1(input));
    }

    #[test]
    fn test_extract_digits_with_words() {
            assert_eq!(('2', '9'), CalibrationLine(String::from("two1nine")).extract_digits_with_words());
            assert_eq!(('8', '3'), CalibrationLine(String::from("eightwothree")).extract_digits_with_words());
            assert_eq!(('1', '3'), CalibrationLine(String::from("abcone2threexyz")).extract_digits_with_words());
            assert_eq!(('2', '4'), CalibrationLine(String::from("xtwone3four")).extract_digits_with_words());
            assert_eq!(('4', '2'), CalibrationLine(String::from("4nineeightseven2")).extract_digits_with_words());
            assert_eq!(('1', '4'), CalibrationLine(String::from("zoneight234")).extract_digits_with_words());
            assert_eq!(('7', '6'), CalibrationLine(String::from("7pqrstsixteen")).extract_digits_with_words());
            assert_eq!(('9', '8'), CalibrationLine(String::from("nineight")).extract_digits_with_words());
    }

    #[test]
    fn test_part2() {
        let input = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
"#;

        assert_eq!(281, calc_part2(input))
    }
}