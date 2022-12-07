pub mod day2 {
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;

    pub fn day2() {
        // read line by line
        if let Ok(lines) = read_lines("src/day2/input.txt") {
            let mut total_score = 0;
            for line in lines {
                if let Ok(game_line) = line {
                    let mut game = Game {
                        player_one_input: dynamic_game_input_from_char_input(
                            game_line.chars().nth(0).unwrap(),
                            game_line.chars().nth(2).unwrap(),
                        ),
                        player_two_input: game_input_from_char(game_line.chars().nth(0).unwrap()),
                        outcome: GameOutcome::WIN,
                    };
                    game.calculate_outcome();
                    let score = game.calculate_score();
                    total_score += score;
                }
            }
            println!("Day 2 Total Score: {}", total_score);
        };
    }

    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    struct Game {
        player_one_input: GameInput,
        player_two_input: GameInput,
        outcome: GameOutcome,
    }

    impl Game {
        fn calculate_score(&self) -> i32 {
            self.outcome as i32 + self.player_one_input as i32
        }

        fn calculate_outcome(&mut self) {
            return match self.player_one_input {
                GameInput::Rock => match self.player_two_input {
                    GameInput::Paper => self.outcome = GameOutcome::LOSS,
                    GameInput::Scissors => self.outcome = GameOutcome::WIN,
                    GameInput::Rock => self.outcome = GameOutcome::DRAW,
                },
                GameInput::Paper => match self.player_two_input {
                    GameInput::Scissors => self.outcome = GameOutcome::LOSS,
                    GameInput::Rock => self.outcome = GameOutcome::WIN,
                    GameInput::Paper => self.outcome = GameOutcome::DRAW,
                },
                GameInput::Scissors => match self.player_two_input {
                    GameInput::Rock => self.outcome = GameOutcome::LOSS,
                    GameInput::Paper => self.outcome = GameOutcome::WIN,
                    GameInput::Scissors => self.outcome = GameOutcome::DRAW,
                },
            };
        }
    }

    #[derive(Copy, Clone)]
    enum GameInput {
        Rock = 1,
        Paper = 2,
        Scissors = 3,
    }

    #[derive(Copy, Clone)]
    enum GameOutcome {
        WIN = 6,
        LOSS = 0,
        DRAW = 3,
    }

    fn game_input_from_char(input: char) -> GameInput {
        return match input {
            'A' | 'X' => GameInput::Rock,
            'B' | 'Y' => GameInput::Paper,
            'C' | 'Z' => GameInput::Scissors,
            _ => panic!("not possible"),
        };
    }

    fn dynamic_game_input_from_char_input(
        input_opponent: char,
        desired_outcome: char,
    ) -> GameInput {
        let game_input_opponent = match input_opponent {
            'A' => GameInput::Rock,
            'B' => GameInput::Paper,
            'C' => GameInput::Scissors,
            _ => panic!("not possible"),
        };
        let desired_game_outcome = match desired_outcome {
            'X' => GameOutcome::LOSS,
            'Y' => GameOutcome::DRAW,
            'Z' => GameOutcome::WIN,
            _ => panic!("not possible"),
        };
        return dynamic_game_input(game_input_opponent, desired_game_outcome);
    }

    fn dynamic_game_input(
        input_opponent: GameInput,
        desired_game_outcome: GameOutcome,
    ) -> GameInput {
        return match input_opponent {
            GameInput::Paper => match desired_game_outcome {
                GameOutcome::WIN => GameInput::Scissors,
                GameOutcome::DRAW => GameInput::Paper,
                GameOutcome::LOSS => GameInput::Rock,
            },
            GameInput::Rock => match desired_game_outcome {
                GameOutcome::WIN => GameInput::Paper,
                GameOutcome::DRAW => GameInput::Rock,
                GameOutcome::LOSS => GameInput::Scissors,
            },
            GameInput::Scissors => match desired_game_outcome {
                GameOutcome::WIN => GameInput::Rock,
                GameOutcome::DRAW => GameInput::Scissors,
                GameOutcome::LOSS => GameInput::Paper,
            },
        };
    }
}

pub fn day2() {
    day2::day2()
}
