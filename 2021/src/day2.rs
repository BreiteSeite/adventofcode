use crate::submarine::{control::parse_control_inputs, Submarine};

pub fn day2() {
    println!("Day 2 | Puzzle 1: {}", day2_puzzle1())
}

fn day2_puzzle1() -> u32 {
    let control_inputs = parse_control_inputs("input/day2_dive");
    let mut submarine = Submarine::default();

    for input in control_inputs {
        submarine.apply_control_input(input);
    }

    submarine.position.horizontal * submarine.position.depth
}

#[cfg(test)]
mod tests {
    use crate::day2::day2_puzzle1;

    #[test]
    fn day2_puzzle2_solution() {
        assert_eq!(1427868, day2_puzzle1());
    }
}
