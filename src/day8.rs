use std::{
    collections::{BTreeSet, HashSet},
    convert::TryFrom,
    str::FromStr,
};
#[derive(Debug, PartialEq)]
enum Command {
    Nop,
    Acc,
    Jmp,
}
#[derive(Debug)]
struct Instruction {
    command: Command,
    arg: i32,
}
impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let [instruction, argument] = s.split(' ').collect::<Vec<_>>()[..] {
            let arg = argument
                .parse()
                .or(Err(ParseError("Could not parse arg")))?;
            let command = match instruction {
                "nop" => Ok(Command::Nop),
                "acc" => Ok(Command::Acc),
                "jmp" => Ok(Command::Jmp),
                _ => Err(ParseError("Unknown instruction")),
            }?;
            Ok(Instruction { command, arg })
        } else {
            Err(ParseError("Could not parse instruction"))
        }
    }
}
#[derive(Debug)]
struct Program(Vec<Instruction>);
#[derive(Debug)]
struct ParseError(&'static str);
enum ProgramResult {
    Terminates(i32),
    Loops(i32),
}
impl Program {
    fn from_string(input: Vec<String>) -> Result<Program, ParseError> {
        let instructions = input
            .iter()
            .map(|x| x.parse())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Program(instructions))
    }
    fn run(&self) -> ProgramResult {
        let mut pointer = 0;
        let mut acc = 0;
        let mut visited = BTreeSet::new();
        loop {
            if pointer == self.0.len() {
                return ProgramResult::Terminates(acc);
            } else if visited.contains(&pointer) {
                return ProgramResult::Loops(acc);
            } else {
                visited.insert(pointer);
            }
            match self.0[pointer] {
                Instruction {
                    command: Command::Nop,
                    arg: _,
                } => {
                    pointer += 1;
                }
                Instruction {
                    command: Command::Acc,
                    arg,
                } => {
                    acc += arg;
                    pointer += 1;
                }
                Instruction {
                    command: Command::Jmp,
                    arg,
                } => {
                    pointer = usize::try_from(pointer as i32 + arg).unwrap();
                }
            };
        }
    }
    fn heal(&mut self) {
        let mut heal_pointer = 0;
        loop {
            if self.0[heal_pointer].command == Command::Nop {
                self.0[heal_pointer].command = Command::Jmp;
                if let ProgramResult::Terminates(_) = self.run() {
                    return ();
                } else {
                    self.0[heal_pointer].command = Command::Nop;
                }
            } else if self.0[heal_pointer].command == Command::Jmp {
                self.0[heal_pointer].command = Command::Nop;
                if let ProgramResult::Terminates(_) = self.run() {
                    return ();
                } else {
                    self.0[heal_pointer].command = Command::Jmp;
                }
            }
            heal_pointer += 1;
        }
    }
}

fn puzzle1(input: Vec<String>) -> i32 {
    if let ProgramResult::Loops(result) = Program::from_string(input).unwrap().run() {
        result
    } else {
        panic!("Did not loop")
    }
}
fn puzzle2(input: Vec<String>) -> i32 {
    let mut program = Program::from_string(input).unwrap();
    program.heal();
    if let ProgramResult::Terminates(result) = program.run() {
        result
    } else {
        panic!("Does not terminate")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_puzzle_test_input() {
        assert_eq!(
            puzzle1(TEST_INPUT.iter().map(|x| x.to_string()).collect()),
            5
        );
    }

    #[test]
    fn first_puzzle_real_input() {
        assert_eq!(
            puzzle1(crate::util::read_file("./data/day8.txt").unwrap()),
            1394
        );
    }

    #[test]
    fn second_puzzle_test_input() {
        assert_eq!(
            puzzle2(TEST_INPUT.iter().map(|x| x.to_string()).collect()),
            8
        );
    }

    #[test]
    fn second_puzzle_real_input() {
        assert_eq!(
            puzzle2(crate::util::read_file("./data/day8.txt").unwrap()),
            1626
        );
    }

    const TEST_INPUT: &'static [&'static str] = &[
        "nop +0", "acc +1", "jmp +4", "acc +3", "jmp -3", "acc -99", "acc +1", "jmp -4", "acc +6",
    ];
}
