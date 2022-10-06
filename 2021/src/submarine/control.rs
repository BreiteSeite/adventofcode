use std::fs::read_to_string;

#[derive(Debug, PartialEq)]
pub enum ControlInput {
    Lower(i32),
    Raise(i32),
    Forward(i32),
}

pub fn parse_control_inputs(file: &str) -> Vec<ControlInput> {
    let mut result = Vec::new();
    for line in read_to_string(file).expect("could not load file").lines() {
        result.push(parse_control_input(line));
    }

    result
}

fn parse_control_input(instruction: &str) -> ControlInput {
    let mut parts = instruction.split_whitespace();
    // assert_eq!(2, parts.count());
    let instruction = parts.next().expect("could not find instruction");
    let value = parts.next().expect("could not find value");
    let value_parsed = value.parse().expect("could not parse value");

    match instruction {
        "up" => ControlInput::Raise(value_parsed),
        "down" => ControlInput::Lower(value_parsed),
        "forward" => ControlInput::Forward(value_parsed),
        _ => panic!("unexpected instruction"),
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_control_input, ControlInput};

    #[test]
    fn parse_up() {
        assert_eq!(ControlInput::Raise(10), parse_control_input("up 10"));
    }

    #[test]
    fn parse_down() {
        assert_eq!(ControlInput::Lower(10), parse_control_input("down 10"));
    }

    #[test]
    fn parse_forward() {
        assert_eq!(ControlInput::Forward(10), parse_control_input("forward 10"));
    }
}
