struct Validator<'a> {
    prelude: &'a [usize],
}

impl Validator<'_> {
    fn new(prelude: &[usize]) -> Validator {
        Validator { prelude }
    }

    fn validate(&self, input: usize) -> bool {
        for i in self.prelude.iter() {
            for j in self.prelude.iter().filter(|&x| x != i) {
                if i + j == input {
                    return true;
                }
            }
        }
        return false;
    }
}

fn find_first_invalid(input: &Vec<usize>, prelude_length: usize) -> usize {
    let mut i = prelude_length;
    loop {
        if !(Validator::new(&input[(i - prelude_length)..i]).validate(input[i])) {
            break input[i];
        } else {
            i += 1;
        }
    }
}

fn find_contiguous_set<'a>(input: &'a Vec<usize>, target: usize) -> &'a [usize] {
    let mut i = 0;
    loop {
        let mut j = i + 1;
        let maybe_result = loop {
            if j == input.len() {
                break None;
            } else {
                let set_candidate = &input[i..(j + 1)];
                let sum: usize = set_candidate.iter().sum();
                if sum == target {
                    break Some(set_candidate);
                } else if sum > target {
                    break None;
                } else {
                    j += 1;
                }
            }
        };
        if let Some(result) = maybe_result {
            break result;
        } else {
            i += 1;
        }
    }
}

pub fn puzzle1(input: Vec<String>, prelude_length: usize) -> usize {
    let input_numbers = input.iter().map(|x| x.parse().unwrap()).collect();
    find_first_invalid(&input_numbers, prelude_length)
}

pub fn puzzle2(input: Vec<String>, prelude_length: usize) -> usize {
    let input_numbers = input.iter().map(|x| x.parse().unwrap()).collect();
    let first_invalid = find_first_invalid(&input_numbers, prelude_length);
    let contiguous_set = find_contiguous_set(&input_numbers, first_invalid);
    contiguous_set.iter().max().unwrap() + contiguous_set.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_puzzle_test_input() {
        assert_eq!(
            puzzle1(TEST_INPUT.iter().map(|x| x.to_string()).collect(), 5),
            127
        );
    }

    #[test]
    fn first_puzzle_real_input() {
        assert_eq!(
            puzzle1(crate::util::read_file("./data/day9.txt").unwrap(), 25),
            70639851
        );
    }

    #[test]
    fn second_puzzle_test_input() {
        assert_eq!(
            puzzle2(TEST_INPUT.iter().map(|x| x.to_string()).collect(), 5),
            62
        );
    }

    #[test]
    fn second_puzzle_real_input() {
        assert_eq!(
            puzzle2(crate::util::read_file("./data/day9.txt").unwrap(), 25),
            8249240
        );
    }

    const TEST_INPUT: &'static [&'static str] = &[
        "35", "20", "15", "25", "47", "40", "62", "55", "65", "95", "102", "117", "150", "182",
        "127", "219", "299", "277", "309", "576",
    ];
}
