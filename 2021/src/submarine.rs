use self::control::ControlInput;

pub mod control;
pub mod navigation;
pub mod sonar;

#[derive(Default)]
pub struct Submarine {
    pub position: navigation::Position,
}

impl Submarine {
    pub fn apply_control_input(&mut self, input: ControlInput) {
        match input {
            ControlInput::Raise(by) => self.position.decrease_depth(by),
            ControlInput::Lower(by) => self.position.increase_depth(by),
            ControlInput::Forward(by) => self.position.forward(by),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Submarine;

    #[test]
    fn control_submarine() {
        let mut submarine = Submarine::default();

        submarine.apply_control_input(super::control::ControlInput::Lower(100));
        submarine.apply_control_input(super::control::ControlInput::Raise(5));
        submarine.apply_control_input(super::control::ControlInput::Forward(25));
        submarine.apply_control_input(super::control::ControlInput::Forward(10));

        assert_eq!(submarine.position.depth, 95);
        assert_eq!(submarine.position.horizontal, 35);
    }
}
