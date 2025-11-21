const INPUT_FILE: &str = "src/aoc/y2016/day10.txt";

use std::{collections::HashMap, fs};

pub fn day10_fst() -> usize {
    let content =
        fs::read_to_string(INPUT_FILE).expect("Could not read input file day2016/day10.txt");
    let instructions = parse_content(&content);
    let transfers = filter_transfers(&instructions);

    let mut game = Game::new();

    for instruction in &instructions {
        if let Instruction::Set { bot_id, chip_value } = instruction {
            game.give_chip(*bot_id, *chip_value);
        }
    }

    if let Some(bot_id) = game.execute(&transfers) {
        bot_id
    } else {
        0
    }
}

pub fn day10_snd() -> i32 {
    let content =
        fs::read_to_string(INPUT_FILE).expect("Could not read input file day2016/day10.txt");
    let instructions = parse_content(&content);
    let transfers = filter_transfers(&instructions);

    let mut game = Game::new();

    for instruction in &instructions {
        if let Instruction::Set { bot_id, chip_value } = instruction {
            game.give_chip(*bot_id, *chip_value);
        }
    }

    game.execute_full(&transfers);

    let product = game.outputs.get(0).unwrap_or(&0)
        * game.outputs.get(1).unwrap_or(&0)
        * game.outputs.get(2).unwrap_or(&0);

    product
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
                let temp = entry.0;
                entry.0 = entry.1;
                entry.1 = temp;
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
                if let Instruction::Transfer { from_bot, instructions } = instruction {
                    if let Some(&(low, high)) = self.assignments.get(from_bot) {
                        if low != -1 && high != -1 {
                            if low == 17 && high == 61 {
                                if early_return {
                                    return Some(*from_bot);
                                }
                            }

                            let (low_target, high_target) = instructions;

                            match low_target {
                                Target::GiveLowToBot { to } => {
                                    self.give_chip(*to, low);
                                }
                                Target::GiveLowToOutput { to } => {
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
                                Target::GiveHighToBot { to } => {
                                    self.give_chip(*to, high);
                                }
                                Target::GiveHighToOutput { to } => {
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
    GiveLowToBot { to: usize },
    GiveHighToBot { to: usize },
    GiveLowToOutput { to: usize },
    GiveHighToOutput { to: usize },
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
                Target::GiveLowToBot {
                    to: parts[6].parse().unwrap(),
                }
            } else {
                Target::GiveLowToOutput {
                    to: parts[6].parse().unwrap(),
                }
            };

            let high_target = if parts[10] == "bot" {
                Target::GiveHighToBot {
                    to: parts[11].parse().unwrap(),
                }
            } else {
                Target::GiveHighToOutput {
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
    fn solution_fst() {
        assert_eq!(day10_fst(), 157);
    }

    #[test]
    fn solution_snd() {
        assert_eq!(day10_snd(), 1085);
    }
}
