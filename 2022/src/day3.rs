use std::{char, collections::HashSet, fs::read_to_string};

pub fn day3() {
    let input = read_to_string("input/day3.txt").expect("can not read day3.txt");
    let intersections = input.lines().map(|l| Rucksack(l).compartment_intersects());

    let prio_summed = intersections.fold(0, |accu, chars| {
        accu + chars
            .iter()
            .map(|character| Rucksack::get_priority(*character))
            .sum::<u32>()
    });

    println!("Day 3 | Part 1: {prio_summed}");
}

struct Rucksack<'a>(&'a str);

impl Rucksack<'_> {
    pub fn compartment_intersects<'a>(self: Self) -> Vec<char> {
        let (compartment_left, compartment_right) = self.0.split_at(self.0.chars().count() / 2);

        let hashset_left: HashSet<char> = HashSet::from_iter(compartment_left.chars().into_iter());
        let hashset_right: HashSet<char> =
            HashSet::from_iter(compartment_right.chars().into_iter());

        let mut result: Vec<char> = Vec::new();
        for item in hashset_left.intersection(&hashset_right) {
            let intersect_char = item.clone();
            result.push(intersect_char);
        }
        result
    }

    pub fn get_priority(char: char) -> u32 {
        // A=65 a=97
        let codepoint = char as u32;

        if char.is_ascii_uppercase() {
            return codepoint - 65 + 27;
        } else if char.is_ascii_lowercase() {
            return codepoint as u32 - 96;
        } else {
            panic!("invalid character");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Rucksack;

    #[test]
    fn test_intersection() {
        let rucksack = Rucksack("vJrwpWtwJgWrhcsFMMfFFhFp");
        assert_eq!(vec!['p'], rucksack.compartment_intersects());
    }

    #[test]
    fn test_priority() {
        assert_eq!(1, Rucksack::get_priority('a'));
        assert_eq!(27, Rucksack::get_priority('A'));
    }
}
