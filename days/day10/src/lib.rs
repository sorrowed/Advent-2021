fn is_opening_brace(c: char) -> bool {
    c == '(' || c == '[' || c == '{' || c == '<'
}

fn matching_brace(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("Invalid character : {}", c),
    }
}

fn syntax_score(c: char) -> i64 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("Invalid character : {}", c),
    }
}

fn total_syntax_score(invalid_tokens: &[char]) -> i64 {
    invalid_tokens
        .iter()
        .fold(0i64, |acc, e| acc + syntax_score(*e))
}

fn autocomplete_score(c: char) -> i64 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("Invalid character : {}", c),
    }
}

fn total_autocomplete_score(autocomplete_tokens: &[char]) -> i64 {
    autocomplete_tokens
        .iter()
        .fold(0i64, |acc, e| acc * 5 + autocomplete_score(*e))
}

fn collect_invalid_tokens(input: &[&str]) -> Vec<char> {
    let mut result = vec![];

    for s in input {
        let mut stack = vec![];

        for c in s.chars() {
            if !stack.is_empty() && c == matching_brace(*stack.last().expect("msg")) {
                stack.pop();
            } else if is_opening_brace(c) {
                stack.push(c);
            } else {
                result.push(c);
                break;
            }
        }
    }
    result
}

fn collect_autocomplete_line(input: &str) -> Option<Vec<char>> {
    let mut stack = vec![];

    for c in input.chars() {
        if !stack.is_empty() && c == matching_brace(*stack.last().expect("msg")) {
            stack.pop();
        } else if is_opening_brace(c) {
            stack.push(c);
        } else {
            return None;
        }
    }

    // Replace all lines with their closing sequence of characters
    stack.reverse();
    Some(stack.into_iter().map(matching_brace).collect())
}

fn part1() {
    let s = std::fs::read_to_string("days/day10/input.txt").unwrap();
    let input = s.split_terminator('\n').collect::<Vec<_>>();

    let score = total_syntax_score(&collect_invalid_tokens(&input));

    println!("Day 10 part 1 : Total syntax score {}", score);
}

fn part2() {
    let s = std::fs::read_to_string("days/day10/input.txt").unwrap();
    let input = s.split_terminator('\n').collect::<Vec<_>>();

    let mut autocomplete = vec![];

    for line in input {
        if let Some(ac) = collect_autocomplete_line(line) {
            autocomplete.push(ac);
        }
    }
    autocomplete.sort_by_key(|a| total_autocomplete_score(a));

    println!(
        "Day 10 part 2 : Middle autocomplete score {}",
        total_autocomplete_score(&autocomplete[autocomplete.len() / 2])
    );
}

pub fn run() {
    part1();
    part2();
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::{
        collect_autocomplete_line, collect_invalid_tokens, total_autocomplete_score,
        total_syntax_score,
    };

    static TEST_INPUT: &[&str] = &[
        "[({(<(())[]>[[{[]{<()<>>",
        "[(()[<>])]({[<{<<[]>>(",
        "{([(<{}[<>[]}>{[]{[(<()>",
        "(((({<>}<{<{<>}{[]{[]{}",
        "[[<[([]))<([[{}[[()]]]",
        "[{[{({}]{}}([{[{{{}}([]",
        "{<[[]]>}<{[{[{[]{()[[[]",
        "[<(<(<(<{}))><([]([]()",
        "<{([([[(<>()){}]>(<<{{",
        "<{([{{}}[<[[[<>{}]]]>[]]",
    ];

    #[test]
    fn part1_test1() {
        let invalid_tokens = collect_invalid_tokens(TEST_INPUT);

        assert_eq!(invalid_tokens, vec!['}', ')', ']', ')', '>']);

        let score = total_syntax_score(&invalid_tokens);

        assert_eq!(score, 26397);
    }

    #[test]
    fn part2_test1() {
        let mut autocomplete = vec![];

        for line in TEST_INPUT {
            if let Some(ac) = collect_autocomplete_line(line) {
                autocomplete.push(ac);
            }
        }

        assert_eq!(autocomplete.len(), 5);
        assert_eq!(total_autocomplete_score(&autocomplete[0]), 288957);
        assert_eq!(total_autocomplete_score(&autocomplete[1]), 5566);
        assert_eq!(total_autocomplete_score(&autocomplete[2]), 1480781);
        assert_eq!(total_autocomplete_score(&autocomplete[3]), 995444);
        assert_eq!(total_autocomplete_score(&autocomplete[4]), 294);

        autocomplete.sort_by(|a, b| total_autocomplete_score(a).cmp(&total_autocomplete_score(b)));

        assert_eq!(total_autocomplete_score(&autocomplete[0]), 294);
        assert_eq!(total_autocomplete_score(&autocomplete[4]), 1480781);

        assert_eq!(
            total_autocomplete_score(&autocomplete[autocomplete.len() / 2]),
            288957
        );
    }
}
