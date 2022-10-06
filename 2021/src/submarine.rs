use core::panic;

use self::{control::ControlInput, navigation::Position};

pub mod control;
pub mod navigation;
pub mod sonar;

#[derive(Default)]
pub struct Submarine {
    pub position: navigation::Position,
}

impl Submarine {
    pub fn applyControlInput(&mut self, input: ControlInput) {
        match input {
            ControlInput::Raise(by) => self.position.decreaseDepth(by),
            ControlInput::Lower(by) => self.position.increaseDepth(by),
            ControlInput::Forward(by) => self.position.forward(by),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Submarine;

    #[test]
    fn controlSubmarine() {
        let mut submarine = Submarine::default();

        submarine.applyControlInput(super::control::ControlInput::Lower((100)));
        submarine.applyControlInput(super::control::ControlInput::Raise((5)));
        submarine.applyControlInput(super::control::ControlInput::Forward((25)));
        submarine.applyControlInput(super::control::ControlInput::Forward((10)));

        assert_eq!(submarine.position.depth, 95);
        assert_eq!(submarine.position.horizontal, 35);
    }
}
