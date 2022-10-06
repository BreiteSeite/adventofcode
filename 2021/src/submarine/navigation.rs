#[derive(Default)]
pub struct Position {
    pub horizontal: u32,
    pub depth: u32,
}

impl Position {
    pub fn increaseDepth(&mut self, by: u32) {
        self.depth = self.depth + by;
    }

    pub fn decreaseDepth(&mut self, by: u32) {
        self.depth = self.depth - by;
    }

    pub fn forward(&mut self, by: u32) {
        self.horizontal = self.horizontal + by;
    }
}

#[cfg(test)]
mod tests {
    use super::Position;

    #[test]
    fn calculatePosition() {
        let mut position = Position {
            depth: 0,
            horizontal: 0,
        };

        position.increaseDepth(10);
        position.increaseDepth(10);
        position.decreaseDepth(5);
        position.forward(7);

        assert_eq!(position.depth, 15);
        assert_eq!(position.horizontal, 7);
    }
}
