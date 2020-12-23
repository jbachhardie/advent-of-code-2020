struct SeatingSimulator {
    state: Vec<Vec<SeatingSpaceState>>,
}

#[derive(PartialEq, Eq, Debug)]
enum SeatingSpaceState {
    Floor,
    Free,
    Occupied,
}

impl SeatingSimulator {
    fn new(input: Vec<String>) -> SeatingSimulator {
        SeatingSimulator {
            state: input
                .iter()
                .map(|row| {
                    row.chars()
                        .map(|char| match char {
                            '.' => SeatingSpaceState::Floor,
                            'L' => SeatingSpaceState::Free,
                            '#' => SeatingSpaceState::Occupied,
                            _ => panic!("Invalid input"),
                        })
                        .collect()
                })
                .collect(),
        }
    }

    fn run_once(&mut self, use_los: bool) -> bool {
        let mut changes = vec![];
        for i in 0..self.state.len() {
            for j in 0..self.state[i].len() {
                if (self.state[i][j] == SeatingSpaceState::Free)
                    & (!self
                        .adjacent_cells(i, j, use_los)
                        .iter()
                        .any(|x| x == &&SeatingSpaceState::Occupied))
                {
                    changes.push((i, j, SeatingSpaceState::Occupied));
                } else if (self.state[i][j] == SeatingSpaceState::Occupied)
                    & (self
                        .adjacent_cells(i, j, use_los)
                        .iter()
                        .filter(|x| x == &&&SeatingSpaceState::Occupied)
                        .count()
                        >= (if use_los { 5 } else { 4 }))
                {
                    changes.push((i, j, SeatingSpaceState::Free));
                }
            }
        }
        if changes.len() > 0 {
            for (change_i, change_j, new_state) in changes {
                self.state[change_i][change_j] = new_state;
            }
            true
        } else {
            false
        }
    }

    fn adjacent_cells(&self, i: usize, j: usize, use_los: bool) -> Vec<&SeatingSpaceState> {
        [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, 1),
            (0, -1),
            (1, 1),
            (1, 0),
            (1, -1),
        ]
        .iter()
        .filter_map(|(i_offset, j_offset)| {
            let mut i_curr = i as i32;
            let mut j_curr = j as i32;
            loop {
                i_curr += i_offset;
                j_curr += j_offset;
                if (i_curr < 0)
                    | (j_curr < 0)
                    | (i_curr >= self.state.len() as i32)
                    | (j_curr >= self.state[i].len() as i32)
                {
                    break None;
                } else {
                    let state = &self.state[(i_curr as usize)][(j_curr as usize)];
                    if !(state == &SeatingSpaceState::Floor) | !use_los {
                        break Some(state);
                    }
                }
            }
        })
        .collect()
    }

    fn run_until_stable(&mut self, use_los: bool) {
        loop {
            let changed = self.run_once(use_los);
            if !changed {
                break;
            }
        }
    }

    fn count_occupied_seats(&self) -> usize {
        let mut count = 0;
        for row in self.state.iter() {
            count += row
                .iter()
                .filter(|x| x == &&SeatingSpaceState::Occupied)
                .count();
        }
        count
    }
}

pub fn puzzle1(input: Vec<String>) -> usize {
    let mut sim = SeatingSimulator::new(input);
    sim.run_until_stable(false);
    sim.count_occupied_seats()
}
pub fn puzzle2(input: Vec<String>) -> usize {
    let mut sim = SeatingSimulator::new(input);
    sim.run_until_stable(true);
    sim.count_occupied_seats()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_puzzle_test_input() {
        assert_eq!(
            puzzle1(TEST_INPUT.iter().map(|x| x.to_string()).collect()),
            37
        );
    }

    #[test]
    fn first_puzzle_real_input() {
        assert_eq!(
            puzzle1(crate::util::read_file("./data/day11.txt").unwrap()),
            2368
        );
    }

    #[test]
    fn second_puzzle_test_input() {
        assert_eq!(
            puzzle2(TEST_INPUT.iter().map(|x| x.to_string()).collect()),
            26
        );
    }

    #[test]
    fn second_puzzle_real_input() {
        assert_eq!(
            puzzle2(crate::util::read_file("./data/day11.txt").unwrap()),
            2124
        );
    }

    const TEST_INPUT: &'static [&'static str] = &[
        "L.LL.LL.LL",
        "LLLLLLL.LL",
        "L.L.L..L..",
        "LLLL.LL.LL",
        "L.LL.LL.LL",
        "L.LLLLL.LL",
        "..L.L.....",
        "LLLLLLLLLL",
        "L.LLLLLL.L",
        "L.LLLLL.LL",
    ];
}
