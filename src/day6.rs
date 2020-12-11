use std::collections::{HashMap, HashSet};

fn puzzle1(input: Vec<String>) -> usize {
    let mut sets = vec![HashSet::new()];
    for line in input {
        if let Some(current_set) = sets.last_mut() {
            if line == "" {
                sets.push(HashSet::new());
            } else {
                for question in line.chars() {
                    current_set.insert(question);
                }
            }
        }
    }
    sets.iter().map(|x| x.len()).sum()
}

fn puzzle2(input: Vec<String>) -> usize {
    let mut all_answers = vec![HashSet::new()];
    let mut people_in_group = 0;
    let mut current_group_answers = HashMap::new();
    for (i, line) in input.iter().enumerate() {
        if let Some(current_group_valid_answers) = all_answers.last_mut() {
            if line != "" {
                people_in_group += 1;
                for question in line.chars() {
                    match current_group_answers.get_mut(&question) {
                        Some(val) => *val += 1,
                        None => {
                            current_group_answers.insert(question, 1);
                        }
                    }
                }
            };
            if (line == "") | (i == input.len() - 1) {
                for (&question, &number_of_people_with_answer) in current_group_answers.iter() {
                    if number_of_people_with_answer == people_in_group {
                        current_group_valid_answers.insert(question);
                    }
                }
                all_answers.push(HashSet::new());
                people_in_group = 0;
                current_group_answers.clear();
            };
        }
    }
    all_answers.iter().map(|x| x.len()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_puzzle_test_input() {
        assert_eq!(
            puzzle1(TEST_INPUT.iter().map(|x| x.to_string()).collect()),
            11
        );
    }

    #[test]
    fn first_puzzle_real_input() {
        assert_eq!(
            puzzle1(crate::util::read_file("./data/day6.txt").unwrap()),
            6310
        );
    }

    #[test]
    fn second_puzzle_test_input() {
        assert_eq!(
            puzzle2(TEST_INPUT.iter().map(|x| x.to_string()).collect()),
            6
        );
    }

    #[test]
    fn second_puzzle_real_input() {
        assert_eq!(
            puzzle2(crate::util::read_file("./data/day6.txt").unwrap()),
            3193
        );
    }

    const TEST_INPUT: &'static [&'static str] = &[
        "abc", "", "a", "b", "c", "", "ab", "ac", "", "a", "a", "a", "a", "", "b",
    ];
}
