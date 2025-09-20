use std::{collections::HashMap, str::FromStr};

#[derive(Debug)]
struct Rule {
    from: String,
    to: String,
}

struct RuleParseError {}

impl FromStr for Rule {
    type Err = RuleParseError;

    fn from_str(token: &str) -> Result<Self, Self::Err> {
        let (from, to) = token.split_once(" -> ").expect("Eeps");
        Ok(Rule {
            from: from.to_string(),
            to: to.to_string(),
        })
    }
}

fn parse_rules(rule_tokens: &[&str]) -> Vec<Rule> {
    rule_tokens
        .iter()
        .skip(1)
        .flat_map(|&token| str::parse::<Rule>(token))
        .collect::<Vec<_>>()
}

fn extract_pairs(template: &str) -> HashMap<String, i64> {
    let mut result = HashMap::new();

    for r in template.as_bytes().windows(2) {
        let key = std::str::from_utf8(r).unwrap().to_string();

        *result.entry(key).or_default() += 1
    }
    result
}

fn apply_rules(rules: &Vec<Rule>, pairs: &HashMap<String, i64>) -> HashMap<String, i64> {
    let mut result: HashMap<String, i64> = HashMap::new();

    for (pair, count) in pairs {
        if let Some(rule) = rules.iter().find(|r| r.from == *pair) {
            let a = rule.from[0..1].to_string() + &rule.to;
            *result.entry(a).or_default() += *count;

            let b = rule.to.clone() + &rule.from[1..2];
            *result.entry(b).or_default() += *count;
        }
    }

    result
}

fn count_letters(pairs: &HashMap<String, i64>, last: char) -> HashMap<char, i64> {
    let mut result = HashMap::new();

    for p in pairs {
        *result.entry(p.0.chars().nth(0).unwrap()).or_insert(0) += p.1
    }

    // Because we didnt count the last letter of the last pair of the original template, we need to add one there
    // Wer can be sure we didnt count that yet because of all pairs we only counted the first letter
    *result.entry(last).or_insert(0) += 1;

    result
}

fn part1() {
    let input = common::import("days/day14/input.txt");
    let b = input.iter().map(|s| s.as_str()).collect::<Vec<_>>();

    let (template, rule_tokens) = b.split_at(1);
    let rules = parse_rules(rule_tokens);

    let template = template.first().unwrap().to_string();
    let last = template.chars().last().unwrap();
    let mut pairs = extract_pairs(&template);

    for _ in 1..=10 {
        pairs = apply_rules(&rules, &pairs);
    }

    let counts = count_letters(&pairs, last);

    let (min, max) = (
        counts.iter().min_by_key(|e| e.1).unwrap().1,
        counts.iter().max_by_key(|e| e.1).unwrap().1,
    );
    assert_eq!(max - min, 2345);
    println!("Day 14 part 1 : {}", max - min);
}

fn part2() {
    let input = common::import("days/day14/input.txt");
    let b = input.iter().map(|s| s.as_str()).collect::<Vec<_>>();

    let (template, rule_tokens) = b.split_at(1);
    let rules = parse_rules(rule_tokens);
    let template = template.first().unwrap().to_string();
    let last = template.chars().last().unwrap();
    let mut pairs = extract_pairs(&template);

    for _ in 1..=40 {
        pairs = apply_rules(&rules, &pairs);
    }

    let counts = count_letters(&pairs, last);

    let (min, max) = (
        counts.iter().min_by_key(|e| e.1).unwrap().1,
        counts.iter().max_by_key(|e| e.1).unwrap().1,
    );

    assert_eq!(max - min, 2432786807053);
    println!("Day 14 part 2 : {}", max - min);
}

pub fn run() {
    part1();
    part2();
}

#[cfg(test)]
mod tests {
    use crate::{apply_rules, count_letters, extract_pairs, parse_rules};

    static TEST_INPUT1: &[&str] = &[
        "NNCB", "", "CH -> B", "HH -> N", "CB -> H", "NH -> C", "HB -> C", "HC -> B", "HN -> C",
        "NN -> C", "BH -> H", "NC -> B", "NB -> B", "BN -> B", "BB -> N", "BC -> B", "CC -> N",
        "CN -> C",
    ];

    #[test]
    fn part1_test1() {
        let (template, rule_tokens) = TEST_INPUT1.split_at(1);

        let rules = parse_rules(rule_tokens);

        let template = template.first().unwrap().to_string();

        let mut pairs = extract_pairs(&template);

        let last = template.chars().last().unwrap();

        pairs = apply_rules(&rules, &pairs);
        let len = count_letters(&pairs, last).iter().fold(0, |a, v| a + v.1);
        assert_eq!(len, "NCNBCHB".len() as i64);

        pairs = apply_rules(&rules, &pairs);
        let len = count_letters(&pairs, last).iter().fold(0, |a, v| a + v.1);
        assert_eq!(len, "NBCCNBBBCBHCB".len() as i64);

        pairs = apply_rules(&rules, &pairs);
        let len = count_letters(&pairs, last).iter().fold(0, |a, v| a + v.1);
        assert_eq!(len, "NBBBCNCCNBBNBNBBCHBHHBCHB".len() as i64);

        pairs = apply_rules(&rules, &pairs);
        let len = count_letters(&pairs, last).iter().fold(0, |a, v| a + v.1);
        assert_eq!(
            len,
            "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB".len() as i64
        );

        pairs = apply_rules(&rules, &pairs);
        let len = count_letters(&pairs, last).iter().fold(0, |a, v| a + v.1);
        assert_eq!(len, 97);

        for _ in 1..=5 {
            pairs = apply_rules(&rules, &pairs);
        }
        let len = count_letters(&pairs, last).iter().fold(0, |a, v| a + v.1);
        assert_eq!(len, 3073);

        let counts = count_letters(&pairs, last);
        let (min, max) = (
            counts.iter().min_by_key(|e| e.1).unwrap().1,
            counts.iter().max_by_key(|e| e.1).unwrap().1,
        );

        assert_eq!(*min, 161);
        assert_eq!(*max, 1749);
        assert_eq!(max - min, 1588);
    }

    #[test]

    fn part2_test1() {
        let (template, rule_tokens) = TEST_INPUT1.split_at(1);

        let rules = parse_rules(rule_tokens);

        let template = template.first().unwrap().to_string();

        let mut pairs = extract_pairs(&template);

        let last = template.chars().last().unwrap();

        for _ in 1..=10 {
            pairs = apply_rules(&rules, &pairs);
        }

        let counts = count_letters(&pairs, last);

        assert_eq!(*counts.get(&'B').unwrap(), 1749);
        assert_eq!(*counts.get(&'C').unwrap(), 298);
        assert_eq!(*counts.get(&'H').unwrap(), 161);
        assert_eq!(*counts.get(&'N').unwrap(), 865);

        for _ in 1..=30 {
            pairs = apply_rules(&rules, &pairs);
        }

        let counts = count_letters(&pairs, last);

        let (min, max) = (
            counts.iter().min_by_key(|e| e.1).unwrap().1,
            counts.iter().max_by_key(|e| e.1).unwrap().1,
        );

        assert_eq!(max - min, 2188189693529);
    }
}
