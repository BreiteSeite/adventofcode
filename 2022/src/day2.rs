use std::{char, fs::read_to_string};

pub fn day2() {
    let input = read_to_string("input/day2.txt").expect("Can not read input");

    let solution_part1: u32 = input
        .lines()
        .map(|line| {
            (
                EncryptedColumnHandShapes(
                    line.chars().nth(0).expect("can not access first column"),
                ),
                EncryptedColumnHandShapes(
                    line.chars().nth(2).expect("can not access second column"),
                ),
            )
        })
        .map(|(col1, col2)| {
            RPSGame {
                opponent: col1.get_shape().unwrap(),
                own: col2.get_shape().unwrap(),
            }
            .total_score()
        })
        .fold(0, |acc, score| acc + score);

    let solution_part2: u32 = input
        .lines()
        .map(|line| {
            (
                EncryptedColumnHandShapes(
                    line.chars().nth(0).expect("can not access first column"),
                ),
                EncryptedColumnGameResult(
                    line.chars().nth(2).expect("can not access second column"),
                ),
            )
        })
        .map(|(col1, col2)| {
            RPSGameWithResult {
                opponent: col1.get_shape().unwrap(),
                desiredResult: col2
                    .get_result()
                    .expect("could not determine desired game result"),
            }
            .total_score()
        })
        .fold(0, |acc, score| acc + score);

    println!("Day 2 Part 1: {solution_part1}");
    println!("Day 2 Part 2: {solution_part2}");
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum HandShape {
    Rock,
    Scissors,
    Paper,
}

#[derive(Debug, PartialEq)]
enum GameResult {
    Won,
    Lost,
    Draw,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct RPSGame {
    opponent: HandShape,
    own: HandShape,
}

struct RPSGameWithResult {
    opponent: HandShape,
    desiredResult: GameResult,
}

impl RPSGameWithResult {
    pub fn get_hand(opponent: HandShape, desiredResult: GameResult) -> HandShape {
        if desiredResult == GameResult::Draw {
            return opponent;
        }

        if desiredResult == GameResult::Won {
            return match opponent {
                HandShape::Rock => HandShape::Paper,
                HandShape::Scissors => HandShape::Rock,
                HandShape::Paper => HandShape::Scissors,
            };
        } else {
            return match opponent {
                HandShape::Rock => HandShape::Scissors,
                HandShape::Scissors => HandShape::Paper,
                HandShape::Paper => HandShape::Rock,
            };
        }
    }
    pub fn total_score(self: Self) -> u32 {
        return RPSGame {
            opponent: self.opponent,
            own: RPSGameWithResult::get_hand(self.opponent, self.desiredResult),
        }
        .total_score();
    }
}

impl RPSGame {
    pub fn play(self: Self) -> GameResult {
        if self.own == self.opponent {
            return GameResult::Draw;
        }

        match (self.own, self.opponent) {
            (HandShape::Rock, HandShape::Scissors) => GameResult::Won,
            (HandShape::Scissors, HandShape::Paper) => GameResult::Won,
            (HandShape::Paper, HandShape::Rock) => GameResult::Won,
            _ => GameResult::Lost,
        }
    }

    pub fn score_for_handshape(shape: HandShape) -> u32 {
        return match shape {
            HandShape::Rock => 1,
            HandShape::Paper => 2,
            HandShape::Scissors => 3,
        };
    }

    pub fn score_for_gameresult(result: GameResult) -> u32 {
        return match result {
            GameResult::Lost => 0,
            GameResult::Draw => 3,
            GameResult::Won => 6,
        };
    }

    pub fn total_score(self: &Self) -> u32 {
        return RPSGame::score_for_gameresult(self.play()) + RPSGame::score_for_handshape(self.own);
    }
}

struct EncryptedColumnHandShapes(char);

impl EncryptedColumnHandShapes {
    fn get_shape(self: Self) -> Option<HandShape> {
        match self.0 {
            'A' => Some(HandShape::Rock),
            'B' => Some(HandShape::Paper),
            'C' => Some(HandShape::Scissors),
            'X' => Some(HandShape::Rock),
            'Y' => Some(HandShape::Paper),
            'Z' => Some(HandShape::Scissors),
            _ => None,
        }
    }
}

struct EncryptedColumnGameResult(char);

impl EncryptedColumnGameResult {
    fn get_result(self: Self) -> Option<GameResult> {
        match self.0 {
            'X' => Some(GameResult::Lost),
            'Y' => Some(GameResult::Draw),
            'Z' => Some(GameResult::Won),
            _ => None,
        }
    }
}

struct Round(String);

fn get_score(shape: HandShape) -> u8 {
    return match shape {
        HandShape::Rock => 1,
        HandShape::Paper => 2,
        HandShape::Scissors => 3,
    };
}

#[cfg(test)]
mod tests {
    use crate::day2::{get_score, EncryptedColumnHandShapes, HandShape};

    use super::{GameResult, RPSGame};

    #[test]
    fn test_get_score() {
        assert_eq!(1, get_score(HandShape::Rock));
        assert_eq!(2, get_score(HandShape::Paper));
        assert_eq!(3, get_score(HandShape::Scissors));
    }

    #[test]
    fn game() {
        assert_eq!(
            GameResult::Draw,
            RPSGame {
                own: HandShape::Rock,
                opponent: HandShape::Rock
            }
            .play()
        );

        assert_eq!(
            GameResult::Lost,
            RPSGame {
                own: HandShape::Paper,
                opponent: HandShape::Scissors
            }
            .play()
        );

        assert_eq!(
            GameResult::Lost,
            RPSGame {
                own: HandShape::Scissors,
                opponent: HandShape::Rock
            }
            .play()
        );

        assert_eq!(
            GameResult::Won,
            RPSGame {
                own: HandShape::Rock,
                opponent: HandShape::Scissors
            }
            .play()
        );
    }

    #[test]
    fn test_total_scores() {
        assert_eq!(
            8,
            RPSGame {
                own: HandShape::Paper,
                opponent: HandShape::Rock
            }
            .total_score()
        );

        assert_eq!(
            1,
            RPSGame {
                own: HandShape::Rock,
                opponent: HandShape::Paper
            }
            .total_score()
        );

        assert_eq!(
            6,
            RPSGame {
                own: HandShape::Scissors,
                opponent: HandShape::Scissors
            }
            .total_score()
        );
    }

    #[test]
    fn decrypt_column() {
        assert_eq!(
            HandShape::Rock,
            EncryptedColumnHandShapes('A').get_shape().unwrap()
        );
        assert_eq!(
            HandShape::Rock,
            EncryptedColumnHandShapes('X').get_shape().unwrap()
        );
        assert_eq!(
            HandShape::Paper,
            EncryptedColumnHandShapes('B').get_shape().unwrap()
        );
        assert_eq!(
            HandShape::Paper,
            EncryptedColumnHandShapes('Y').get_shape().unwrap()
        );
        assert_eq!(
            HandShape::Scissors,
            EncryptedColumnHandShapes('C').get_shape().unwrap()
        );
        assert_eq!(
            HandShape::Scissors,
            EncryptedColumnHandShapes('Z').get_shape().unwrap()
        );
    }
}
