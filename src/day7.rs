use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use regex::Regex;

#[derive(Debug)]
struct Rules(HashMap<BagType, ContainmentRules>);
#[derive(Debug)]
struct RuleParsingError(&'static str);
impl Rules {
    fn from_strings(strings: Vec<String>) -> Result<Rules, RuleParsingError> {
        let re = Regex::new(r"^(.+) bags? contain (.+)$").unwrap();
        let mut rules_map = HashMap::new();
        for line in strings {
            let maybe_captures = re.captures(line.as_str());
            if let Some(captures) = maybe_captures {
                if let (Some(bag_type), Some(containment_rules)) =
                    (captures.get(1), captures.get(2))
                {
                    rules_map.insert(
                        bag_type.as_str().to_string(),
                        containment_rules.as_str().parse()?,
                    );
                } else {
                    return Err(RuleParsingError("Wrong number of captures"));
                }
            } else {
                return Err(RuleParsingError("Rule does not match regex"));
            }
        }
        Ok(Rules(rules_map))
    }
    fn get_bags_which_can_contain(&self, bag_type: &BagType) -> HashSet<&BagType> {
        let mut result = HashSet::new();
        for (potential_container, rules) in self.0.iter() {
            if let ContainmentRules::Contains(defs) = rules {
                if defs.iter().any(|x| &x.bag_type == bag_type) {
                    result.insert(potential_container);
                    for second_order_result in
                        self.get_bags_which_can_contain(potential_container).iter()
                    {
                        result.insert(second_order_result);
                    }
                };
            };
        }
        result
    }
    fn get_number_of_bags_contained_in(&self, bag_type: &BagType) -> usize {
        match self.0.get(bag_type) {
            None => 0,
            Some(ContainmentRules::ContainsNothing) => 0,
            Some(ContainmentRules::Contains(definitions)) => {
                definitions.iter().fold(0, |acc, item| {
                    acc + item.amount * (self.get_number_of_bags_contained_in(&item.bag_type) + 1)
                })
            }
        }
    }
}
type BagType = String;
#[derive(Debug)]
enum ContainmentRules {
    ContainsNothing,
    Contains(Vec<ContainmentDefinition>),
}
impl FromStr for ContainmentRules {
    type Err = RuleParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "no other bags." {
            Ok(ContainmentRules::ContainsNothing)
        } else {
            Ok(ContainmentRules::Contains(
                s.split(',')
                    .map(|rule_string| rule_string.parse())
                    .collect::<Result<_, _>>()?,
            ))
        }
    }
}
#[derive(Debug)]
struct ContainmentDefinition {
    amount: usize,
    bag_type: BagType,
}
impl FromStr for ContainmentDefinition {
    type Err = RuleParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"(\d+) (.+) bag").unwrap();
        let maybe_matches = re.captures(s);
        if let Some(matches) = maybe_matches {
            if let (Some(amount_match), Some(bag_type_match)) = (matches.get(1), matches.get(2)) {
                if let Ok(amount) = amount_match.as_str().parse() {
                    Ok(ContainmentDefinition {
                        amount,
                        bag_type: bag_type_match.as_str().to_string(),
                    })
                } else {
                    Err(RuleParsingError("Amount did not parse as number"))
                }
            } else {
                panic!("Wrong number of matches")
            }
        } else {
            Err(RuleParsingError(
                "Containment definition did not match regex",
            ))
        }
    }
}

pub fn puzzle1(input: Vec<String>) -> usize {
    Rules::from_strings(input)
        .unwrap()
        .get_bags_which_can_contain(&"shiny gold".to_string())
        .iter()
        .count()
}

pub fn puzzle2(input: Vec<String>) -> usize {
    Rules::from_strings(input)
        .unwrap()
        .get_number_of_bags_contained_in(&"shiny gold".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_puzzle_test_input() {
        assert_eq!(
            puzzle1(TEST_INPUT.iter().map(|x| x.to_string()).collect()),
            4
        );
    }

    #[test]
    fn first_puzzle_real_input() {
        assert_eq!(
            puzzle1(crate::util::read_file("./data/day7.txt").unwrap()),
            128
        );
    }

    #[test]
    fn second_puzzle_test_input() {
        assert_eq!(
            puzzle2(TEST_INPUT.iter().map(|x| x.to_string()).collect()),
            32
        );
    }

    #[test]
    fn second_puzzle_test_input_2() {
        assert_eq!(
            puzzle2(TEST_INPUT_2.iter().map(|x| x.to_string()).collect()),
            126
        );
    }

    #[test]
    fn second_puzzle_real_input() {
        assert_eq!(
            puzzle2(crate::util::read_file("./data/day7.txt").unwrap()),
            20189
        );
    }

    const TEST_INPUT: &'static [&'static str] = &[
        "light red bags contain 1 bright white bag, 2 muted yellow bags.",
        "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
        "bright white bags contain 1 shiny gold bag.",
        "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
        "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
        "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
        "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
        "faded blue bags contain no other bags.",
        "dotted black bags contain no other bags.",
    ];

    const TEST_INPUT_2: &'static [&'static str] = &[
        "shiny gold bags contain 2 dark red bags.",
        "dark red bags contain 2 dark orange bags.",
        "dark orange bags contain 2 dark yellow bags.",
        "dark yellow bags contain 2 dark green bags.",
        "dark green bags contain 2 dark blue bags.",
        "dark blue bags contain 2 dark violet bags.",
        "dark violet bags contain no other bags.",
    ];
}
