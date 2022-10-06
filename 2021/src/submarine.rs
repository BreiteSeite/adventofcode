use self::control::ControlInput;

pub mod control;
pub mod navigation;
pub mod sonar;

#[derive(Default)]
pub struct Submarine {
    pub position: navigation::Position,
    pub aim: i32,
}

impl Submarine {
    pub fn navigate(&mut self, input: ControlInput) {
        match input {
            ControlInput::Raise(by) => self.position.decrease_depth(by),
            ControlInput::Lower(by) => self.position.increase_depth(by),
            ControlInput::Forward(by) => self.position.forward(by),
        }
    }

    pub fn navigate_with_aim(&mut self, input: ControlInput) {
        match input {
            ControlInput::Raise(by) => {
                self.aim = self.aim - by;
            }
            ControlInput::Lower(by) => {
                self.aim = self.aim + by;
            }
            ControlInput::Forward(by) => {
                self.position.forward(by);
                self.position.increase_depth(self.aim * by)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Submarine;

    #[test]
    fn navigate() {
        let mut submarine = Submarine::default();

        submarine.navigate(super::control::ControlInput::Lower(100));
        submarine.navigate(super::control::ControlInput::Raise(5));
        submarine.navigate(super::control::ControlInput::Forward(25));
        submarine.navigate(super::control::ControlInput::Forward(10));

        assert_eq!(submarine.position.depth, 95);
        assert_eq!(submarine.position.horizontal, 35);
    }

    #[test]
    fn navigate_with_aim() {
        let mut submarine = Submarine::default();

        submarine.navigate_with_aim(super::control::ControlInput::Forward(5));
        submarine.navigate_with_aim(super::control::ControlInput::Lower(5));
        submarine.navigate_with_aim(super::control::ControlInput::Forward(8));
        submarine.navigate_with_aim(super::control::ControlInput::Raise(3));
        submarine.navigate_with_aim(super::control::ControlInput::Lower(8));
        submarine.navigate_with_aim(super::control::ControlInput::Forward(2));

        assert_eq!(submarine.position.depth, 60);
        assert_eq!(submarine.position.horizontal, 15);
    }
}
