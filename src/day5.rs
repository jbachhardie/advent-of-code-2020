use std::str::FromStr;

#[derive(Debug)]
struct BoardingPass {
    row_instructions: [RowInstruction; 7],
    column_instructions: [ColumnInstruction; 3],
}

#[derive(Debug)]
struct BoardingPassParseError {
    reason: &'static str,
}

impl FromStr for BoardingPass {
    type Err = BoardingPassParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(BoardingPass {
            row_instructions: s[..7]
                .chars()
                .map(|x| x.to_string().parse())
                .collect::<Result<Vec<_>, _>>()?
                .iter()
                .enumerate()
                .fold([RowInstruction::Front; 7], |mut acc, (i, &x)| {
                    if i < acc.len() {
                        acc[i] = x;
                    }
                    acc
                }),
            column_instructions: s[7..]
                .chars()
                .map(|x| x.to_string().parse())
                .collect::<Result<Vec<_>, _>>()?
                .iter()
                .enumerate()
                .fold([ColumnInstruction::Left; 3], |mut acc, (i, &x)| {
                    if i < acc.len() {
                        acc[i] = x;
                    }
                    acc
                }),
        })
    }
}

trait ToSeatingPosition {
    fn to_seating_position(&self) -> SeatingPosition;
}

impl ToSeatingPosition for BoardingPass {
    fn to_seating_position(&self) -> SeatingPosition {
        fn calculate_row(instructions: std::slice::Iter<RowInstruction>) -> usize {
            instructions
                .fold((0, 127), |(lower, upper), item| {
                    let span = upper - lower;
                    if span > 1 {
                        match item {
                            RowInstruction::Back => (lower + span / 2 + 1, upper),
                            RowInstruction::Front => (lower, lower + span / 2),
                        }
                    } else {
                        match item {
                            RowInstruction::Back => (upper, upper),
                            RowInstruction::Front => (lower, lower),
                        }
                    }
                })
                .0
        }
        fn calculate_column(instructions: std::slice::Iter<ColumnInstruction>) -> usize {
            instructions
                .fold((0, 7), |(lower, upper), item| {
                    let span = upper - lower;
                    if span > 1 {
                        match item {
                            ColumnInstruction::Right => (lower + span / 2 + 1, upper),
                            ColumnInstruction::Left => (lower, lower + span / 2),
                        }
                    } else {
                        match item {
                            ColumnInstruction::Right => (upper, upper),
                            ColumnInstruction::Left => (lower, lower),
                        }
                    }
                })
                .0
        }
        let row = calculate_row(self.row_instructions.iter());
        let column = calculate_column(self.column_instructions.iter());
        SeatingPosition {
            row,
            column,
            id: row * 8 + column,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum RowInstruction {
    Front,
    Back,
}

impl FromStr for RowInstruction {
    type Err = BoardingPassParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "F" => Ok(RowInstruction::Front),
            "B" => Ok(RowInstruction::Back),
            _ => Err(BoardingPassParseError {
                reason: "Unknown Row Instruction",
            }),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum ColumnInstruction {
    Left,
    Right,
}

impl FromStr for ColumnInstruction {
    type Err = BoardingPassParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(ColumnInstruction::Left),
            "R" => Ok(ColumnInstruction::Right),
            _ => Err(BoardingPassParseError {
                reason: "Unknown Column Instruction",
            }),
        }
    }
}

#[derive(Debug)]
struct SeatingPosition {
    row: usize,
    column: usize,
    id: usize,
}

pub fn puzzle1(input: Vec<String>) -> usize {
    input
        .iter()
        .map(|x| x.parse::<BoardingPass>())
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
        .iter()
        .map(|x| x.to_seating_position().id)
        .max()
        .unwrap()
}
pub fn puzzle2(input: Vec<String>) -> usize {
    let mut all_boarding_passes = input
        .iter()
        .map(|x| x.parse::<BoardingPass>())
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
        .iter()
        .map(|x| x.to_seating_position().id)
        .collect::<Vec<_>>();
    all_boarding_passes.sort_unstable();
    let mut prev = all_boarding_passes.remove(0);
    for id in all_boarding_passes.iter() {
        if id - prev == 2 {
            return prev + 1;
        } else {
            prev = *id;
        }
    }
    panic!("No ID found")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_puzzle_test_input() {
        assert_eq!(
            puzzle1(TEST_INPUT.iter().map(|x| x.to_string()).collect()),
            820
        );
    }

    #[test]
    fn first_puzzle_real_input() {
        assert_eq!(
            puzzle1(crate::util::read_file("./data/day5.txt").unwrap()),
            848
        );
    }

    #[test]
    fn second_puzzle_real_input() {
        assert_eq!(
            puzzle2(crate::util::read_file("./data/day5.txt").unwrap()),
            682
        );
    }

    const TEST_INPUT: &'static [&'static str] =
        &["FBFBBFFRLR", "BFFFBBFRRR", "FFFBBBFRRR", "BBFFBBFRLL"];
}
