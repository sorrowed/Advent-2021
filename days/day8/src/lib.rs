use itertools::Itertools;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

#[allow(non_camel_case_types)]
#[derive(Debug, PartialOrd, Ord, Eq, PartialEq)]
enum Segment {
    a,
    b,
    c,
    d,
    e,
    f,
    g,
}

#[derive(Debug)]
struct ParseSegmentError;
impl TryFrom<char> for Segment {
    type Error = ParseSegmentError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'a' => Ok(Segment::a),
            'b' => Ok(Segment::b),
            'c' => Ok(Segment::c),
            'd' => Ok(Segment::d),
            'e' => Ok(Segment::e),
            'f' => Ok(Segment::f),
            'g' => Ok(Segment::g),
            _ => Err(ParseSegmentError),
        }
    }
}

fn segment_permutations_lookup() -> Vec<HashMap<char, char>> {
    let segments = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'];

    segments
        .iter()
        .permutations(segments.len())
        .map(|perm| {
            segments
                .iter()
                .cloned()
                .zip(perm.into_iter().cloned())
                .collect()
        })
        .collect()
}

/// A tuple struct that wraps a Vec<Segment> to be able to implement From<&str> for Segments
#[derive(Debug)]
struct Segments(Vec<Segment>);

impl Segments {
    /// If a digit requires these number of segments to be on, we know which digits they are:
    /// 2 segments = "1"
    /// 3 segments = "7"
    /// 4 segments = "4"
    /// 7 segments = "8"
    pub fn has_unique_number_of_segments(&self) -> bool {
        self.len() == 2 || self.len() == 3 || self.len() == 4 || self.len() == 7
    }

    pub fn to_digit(&self) -> Option<i32> {
        match self.as_slice() {
            [Segment::a, Segment::b, Segment::c, Segment::e, Segment::f, Segment::g] => Some(0),
            [Segment::c, Segment::f] => Some(1),
            [Segment::a, Segment::c, Segment::d, Segment::e, Segment::g] => Some(2),
            [Segment::a, Segment::c, Segment::d, Segment::f, Segment::g] => Some(3),
            [Segment::b, Segment::c, Segment::d, Segment::f] => Some(4),
            [Segment::a, Segment::b, Segment::d, Segment::f, Segment::g] => Some(5),
            [Segment::a, Segment::b, Segment::d, Segment::e, Segment::f, Segment::g] => Some(6),
            [Segment::a, Segment::c, Segment::f] => Some(7),
            [Segment::a, Segment::b, Segment::c, Segment::d, Segment::e, Segment::f, Segment::g] => {
                Some(8)
            }
            [Segment::a, Segment::b, Segment::c, Segment::d, Segment::f, Segment::g] => Some(9),
            _ => None,
        }
    }

    pub fn to_digits(segments: &Vec<Segments>) -> Vec<i32> {
        segments
            .iter()
            .filter_map(|p| p.to_digit())
            .collect::<Vec<_>>()
    }
}

impl std::ops::Deref for Segments {
    type Target = Vec<Segment>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&str> for Segments {
    fn from(value: &str) -> Self {
        let mut s = value
            .chars()
            .map(|c| Segment::try_from(c).expect("Invalid segment name"))
            .collect::<Vec<_>>();

        s.sort();

        Segments(s)
    }
}

struct Entry(Vec<Segments>, Vec<Segments>);

impl Entry {
    // The only one that translates the signal patterns to valid digits in range 0 ..= 9
    // These should then be usable to translate the digits in the second field
    pub fn to_digits(&self) -> Option<Vec<i32>> {
        let signal_patterns = Segments::to_digits(&self.0);

        if signal_patterns.len() == 10 {
            Some(Segments::to_digits(&self.1))
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct ParseEntryError {
    msg: String,
}

impl ParseEntryError {
    pub fn new(msg: String) -> ParseEntryError {
        ParseEntryError { msg: msg }
    }
}
fn parse_segments(input: &str) -> Vec<Segments> {
    input
        .split_whitespace()
        .map(|s| Segments::from(s))
        .collect()
}

impl FromStr for Entry {
    type Err = ParseEntryError;

    /// It is assumed that s is a line in the form "s s s s | dd ddddd ddddd dddd" where
    /// s are groups of (input) segments and d are groups of segments that represent digits
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.splitn(2, " | ");

        let patterns = tokens.next().ok_or(ParseEntryError::new(format!(
            "Missing pattern in line {}",
            s
        )))?;

        let digits = tokens.next().ok_or(ParseEntryError::new(format!(
            "Missing digits in line {}",
            s
        )))?;

        Ok(Entry(parse_segments(patterns), parse_segments(digits)))
    }
}

fn brute_force(input: &str) -> Option<Vec<i32>> {
    let lut = segment_permutations_lookup();
    for p in lut {
        let altered_input = input
            .chars()
            .map(|c| p.get(&c).cloned().unwrap_or(c))
            .collect::<String>();

        if let Ok(entry) = Entry::from_str(altered_input.as_str()) {
            if let Some(d) = Entry::to_digits(&entry) {
                return Some(d);
            }
        }
    }
    None
}

fn count_unique(input: &[&str]) -> usize {
    input
        .iter()
        .map(|n| {
            Entry::from_str(n)
                .expect("Invalid entry")
                .1
                .iter()
                .filter(|n| n.has_unique_number_of_segments())
                .count()
        })
        .fold(0, |a, i| a + i)
}

fn concatenate_integers(input: &[i32]) -> i32 {
    input.iter().fold(0, |a, e| a * 10 + e)
}

fn part1() {
    let s = std::fs::read_to_string("days/day8/input.txt").unwrap();

    println!(
        "Day 8 part 1 : Unique digits {}",
        count_unique(&s.split_terminator("\n").collect::<Vec<_>>())
    );
}

fn part2() {
    let reader =
        BufReader::new(File::open("days/day8/input.txt").expect("Failed to open input file"));

    let s = reader
        .lines()
        .filter_map(|line| brute_force(&line.expect("OMG")))
        .map(|r| concatenate_integers(&r))
        .sum::<i32>();

    assert_eq!(s, 973292);

    println!("Day 8 part 2 : Sum of output values: {}", s);
}

pub fn run() {
    part1();
    part2();
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &[&str] = &[
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
        "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc",
        "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg",
        "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb",
        "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea",
        "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb",
        "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe",
        "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef",
        "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb",
        "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce",
    ];

    #[test]
    fn part1_test() {
        assert_eq!(count_unique(TEST_INPUT), 26);
    }

    #[test]
    fn part2_test1() {
        let input =
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";

        let lut = segment_permutations_lookup();
        for p in lut {
            let altered_input = input
                .chars()
                .map(|c| {
                    if p.contains_key(&c) {
                        p.get(&c)
                            .expect(format!("Invalid mapping {} in {:?}", c, p).as_str())
                            .clone()
                    } else {
                        c
                    }
                })
                .collect::<String>();

            if let Ok(entry) = Entry::from_str(altered_input.as_str()) {
                let signal_patterns = entry
                    .0
                    .iter()
                    .filter_map(|p| p.to_digit())
                    .collect::<Vec<_>>();

                // The only one that translates the signal patterns to valid digits in range 0 ..= 9
                // These should then be usable to translate the digits in the second field
                if signal_patterns.len() == 10 {
                    let digits = entry
                        .1
                        .iter()
                        .filter_map(|e| e.to_digit())
                        .collect::<Vec<_>>();

                    println!("{:?} {:?}", signal_patterns, digits);

                    assert_eq!(digits, vec![5, 3, 5, 3]);
                }
            }
        }
    }

    #[test]
    fn part2_test2() {
        let r = brute_force(
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf",
        );

        assert!(r == Some(vec![5, 3, 5, 3]));

        let required = [8394, 9781, 1197, 9361, 4873, 8418, 4548, 1625, 8717, 4315];
        for (i, input) in TEST_INPUT.iter().enumerate() {
            if let Some(integers) = brute_force(input) {
                let c = concatenate_integers(&integers);

                assert_eq!(c, required[i]);
            }
        }

        let s = TEST_INPUT
            .iter()
            .filter_map(|input| brute_force(input))
            .map(|r| concatenate_integers(&r))
            .sum::<i32>();
        assert_eq!(s, 61229);
    }
}
