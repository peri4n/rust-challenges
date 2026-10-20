pub const INPUT: &str = include_str!("day10.txt");

use std::collections::HashMap;

use crate::aoc::Solution;

pub struct Day10 {
    instructions: Vec<Instruction>,
}

impl Solution for Day10 {
    type Fst = usize;
    type Snd = i32;

    fn parse(input: &str) -> Self {
        Day10 {
            instructions: parse_content(input),
        }
    }

    fn fst(&self) -> usize {
        let transfers = filter_transfers(&self.instructions);

        let mut game = Game::new();

        for instruction in &self.instructions {
            if let Instruction::Set { bot_id, chip_value } = instruction {
                game.give_chip(*bot_id, *chip_value);
            }
        }

        game.execute(&transfers).unwrap_or_default()
    }

    fn snd(&self) -> i32 {
        let transfers = filter_transfers(&self.instructions);

        let mut game = Game::new();

        for instruction in &self.instructions {
            if let Instruction::Set { bot_id, chip_value } = instruction {
                game.give_chip(*bot_id, *chip_value);
            }
        }

        game.execute_full(&transfers);

        game.outputs.first().unwrap_or(&0)
            * game.outputs.get(1).unwrap_or(&0)
            * game.outputs.get(2).unwrap_or(&0)
    }
}

fn filter_transfers(instructions: &Vec<Instruction>) -> Vec<Instruction> {
    let mut transfers = Vec::new();

    for instruction in instructions {
        if matches!(instruction, Instruction::Transfer { .. }) {
            transfers.push(*instruction);
        }
    }
    transfers
}

struct Game {
    assignments: HashMap<usize, (i32, i32)>,
    outputs: Vec<i32>,
}

impl Game {
    pub fn new() -> Self {
        Game {
            assignments: HashMap::new(),
            outputs: Vec::new(),
        }
    }

    pub fn give_chip(&mut self, bot_id: usize, chip_value: i32) {
        let entry = self.assignments.entry(bot_id).or_insert((-1, -1));
        if entry.0 == -1 {
            entry.0 = chip_value;
        } else if entry.1 == -1 {
            entry.1 = chip_value;
            if entry.0 > entry.1 {
                std::mem::swap(&mut entry.0, &mut entry.1);
            }
        }
    }

    pub fn execute(&mut self, transfers: &[Instruction]) -> Option<usize> {
        self.execute_with_early_return(transfers, true)
    }

    pub fn execute_full(&mut self, transfers: &[Instruction]) {
        self.execute_with_early_return(transfers, false);
    }

    fn execute_with_early_return(
        &mut self,
        transfers: &[Instruction],
        early_return: bool,
    ) -> Option<usize> {
        loop {
            let mut made_progress = false;

            for instruction in transfers {
                if let Instruction::Transfer { from_bot, instructions } = instruction
                    && let Some(&(low, high)) = self.assignments.get(from_bot)
                    && low != -1
                    && high != -1
                {
                    if low == 17 && high == 61 && early_return {
                        return Some(*from_bot);
                    }

                    let (low_target, high_target) = instructions;

                    match low_target {
                        Target::LowToBot { to } => {
                            self.give_chip(*to, low);
                        }
                        Target::LowToOutput { to } => {
                            if *to < self.outputs.len() {
                                self.outputs[*to] = low;
                            } else {
                                while self.outputs.len() <= *to {
                                    self.outputs.push(0);
                                }
                                self.outputs[*to] = low;
                            }
                        }
                        _ => {}
                    }

                    match high_target {
                        Target::HighToBot { to } => {
                            self.give_chip(*to, high);
                        }
                        Target::HighToOutput { to } => {
                            if *to < self.outputs.len() {
                                self.outputs[*to] = high;
                            } else {
                                while self.outputs.len() <= *to {
                                    self.outputs.push(0);
                                }
                                self.outputs[*to] = high;
                            }
                        }
                        _ => {}
                    }

                    self.assignments.insert(*from_bot, (-1, -1));
                    made_progress = true;
                }
            }

            if !made_progress {
                break;
            }
        }
        None
    }
}

#[derive(Debug, Copy, Clone)]
enum Target {
    LowToBot { to: usize },
    HighToBot { to: usize },
    LowToOutput { to: usize },
    HighToOutput { to: usize },
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Transfer {
        from_bot: usize,
        instructions: (Target, Target),
    },
    Set {
        bot_id: usize,
        chip_value: i32,
    },
}

fn parse_content(content: &str) -> Vec<Instruction> {
    let mut instructions = Vec::new();

    for line in content.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts[0] == "value" {
            let chip_value: i32 = parts[1].parse().unwrap();
            let bot_id: usize = parts[5].parse().unwrap();
            instructions.push(Instruction::Set { bot_id, chip_value });
        } else if parts[0] == "bot" {
            let from_bot: usize = parts[1].parse().unwrap();

            let low_target = if parts[5] == "bot" {
                Target::LowToBot {
                    to: parts[6].parse().unwrap(),
                }
            } else {
                Target::LowToOutput {
                    to: parts[6].parse().unwrap(),
                }
            };

            let high_target = if parts[10] == "bot" {
                Target::HighToBot {
                    to: parts[11].parse().unwrap(),
                }
            } else {
                Target::HighToOutput {
                    to: parts[11].parse().unwrap(),
                }
            };

            instructions.push(Instruction::Transfer {
                from_bot,
                instructions: (low_target, high_target),
            });
        }
    }

    instructions
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn solution() {
        let day = Day10::parse(INPUT);
        assert_eq!(day.fst(), 157);
        assert_eq!(day.snd(), 1085);
    }
}
