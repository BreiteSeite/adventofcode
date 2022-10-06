use crate::submarine::{control::parse_control_inputs, Submarine};

pub fn day2() {
    println!("Day 2 | Puzzle 1: {}", day2_puzzle1());
    println!("Day 2 | Puzzle 2: {}", day2_puzzle2());
}

fn day2_puzzle1() -> i32 {
    let control_inputs = parse_control_inputs("input/day2_dive");
    let mut submarine = Submarine::default();

    for input in control_inputs {
        submarine.navigate(input);
    }

    submarine.position.horizontal * submarine.position.depth
}

fn day2_puzzle2() -> i32 {
    let control_inputs = parse_control_inputs("input/day2_dive");
    let mut submarine = Submarine::default();

    for input in control_inputs {
        submarine.navigate_with_aim(input);
    }

    submarine.position.horizontal * submarine.position.depth
}

#[cfg(test)]
mod tests {
    use crate::day2::{day2_puzzle1, day2_puzzle2};

    #[test]
    fn day2_puzzle1_solution() {
        assert_eq!(1427868, day2_puzzle1());
    }
    #[test]
    fn day2_puzzle2_solution() {
        assert_eq!(1568138742, day2_puzzle2());
    }
}
