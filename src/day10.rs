fn to_numbers(input: Vec<String>) -> Vec<usize> {
    input.iter().map(|x| x.parse().unwrap()).collect()
}

pub fn puzzle1(input: Vec<String>) -> usize {
    let mut input_numbers = to_numbers(input);
    input_numbers.sort();
    input_numbers.insert(0, 0);
    input_numbers.push(input_numbers[input_numbers.len() - 1] + 3);
    let mut one_jolt_leaps = 0;
    let mut three_jolt_leaps = 0;
    for i in 1..input_numbers.len() {
        let leap = input_numbers[i] - input_numbers[i - 1];
        if leap == 1 {
            one_jolt_leaps += 1;
        } else if leap == 3 {
            three_jolt_leaps += 1;
        } else if leap > 3 {
            panic!("Chain broken");
        }
    }
    one_jolt_leaps * three_jolt_leaps
}

pub fn puzzle2(input: Vec<String>) -> usize {
    let mut input_numbers = to_numbers(input);
    input_numbers.sort();
    input_numbers.insert(0, 0);
    input_numbers.push(input_numbers[input_numbers.len() - 1] + 3);
    let mut combinations = vec![0; input_numbers.len()];
    for i in (0..input_numbers.len()).rev() {
        let mut j = i + 1;
        loop {
            if j >= input_numbers.len() {
                combinations[i] = 1;
                break;
            } else if input_numbers[j] - input_numbers[i] > 3 {
                break;
            }
            combinations[i] += combinations[j];
            j += 1;
        }
    }
    combinations[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_puzzle_test_input() {
        assert_eq!(
            puzzle1(TEST_INPUT.iter().map(|x| x.to_string()).collect()),
            7 * 5
        );
    }

    #[test]
    fn first_puzzle_test_input_2() {
        assert_eq!(
            puzzle1(TEST_INPUT_2.iter().map(|x| x.to_string()).collect()),
            22 * 10
        );
    }

    #[test]
    fn first_puzzle_real_input() {
        assert_eq!(
            puzzle1(crate::util::read_file("./data/day10.txt").unwrap()),
            2380
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
    fn second_puzzle_test_input_2() {
        assert_eq!(
            puzzle2(TEST_INPUT_2.iter().map(|x| x.to_string()).collect()),
            19208
        );
    }

    #[test]
    fn second_puzzle_real_input() {
        assert_eq!(
            puzzle2(crate::util::read_file("./data/day10.txt").unwrap()),
            48358655787008
        );
    }

    const TEST_INPUT: &'static [&'static str] =
        &["16", "10", "15", "5", "1", "11", "7", "19", "6", "12", "4"];
    const TEST_INPUT_2: &'static [&'static str] = &[
        "28", "33", "18", "42", "31", "14", "46", "20", "48", "47", "24", "23", "49", "45", "19",
        "38", "39", "11", "1", "32", "25", "35", "8", "17", "7", "9", "4", "2", "34", "10", "3",
    ];
}
