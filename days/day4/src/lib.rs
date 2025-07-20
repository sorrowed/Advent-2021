use std::collections::HashSet;
use std::fs;

#[derive(Debug)]
struct Loc {
    n: i32,
    m: bool,
}

impl Loc {
    fn new(n: i32) -> Loc {
        Loc { n: n, m: false }
    }
}

#[derive(Debug)]
struct Board {
    rows: Vec<Vec<Loc>>,
    columns: Vec<Vec<Loc>>,
    last_draw: i32,
    round: i32,
}

impl Board {
    fn new(numbers: &Vec<i32>) -> Board {
        Board {
            rows: Self::select_rows(numbers),
            columns: Self::select_columns(numbers),
            last_draw: -1,
            round: -1,
        }
    }

    fn select_rows(numbers: &Vec<i32>) -> Vec<Vec<Loc>> {
        (0..5)
            .map(|i| {
                numbers
                    .iter()
                    .map(|&n| n)
                    .skip(i * 5)
                    .take(5)
                    .map(|n| Loc::new(n))
                    .collect()
            })
            .collect()
    }

    fn select_columns(numbers: &Vec<i32>) -> Vec<Vec<Loc>> {
        (0..5)
            .map(|i| {
                numbers
                    .iter()
                    .map(|&n| n)
                    .skip(i)
                    .step_by(5)
                    .map(|n| Loc::new(n))
                    .collect()
            })
            .collect()
    }

    fn draw(&mut self, number: i32) -> bool {
        for row in &mut self.rows {
            for loc in row {
                loc.m |= loc.n == number
            }
        }
        for column in &mut self.columns {
            for loc in column {
                loc.m |= loc.n == number
            }
        }

        self.is_winner()
    }

    fn is_winning_line(line: &Vec<Loc>) -> bool {
        line.iter().all(|l| l.m)
    }

    fn is_winner(&self) -> bool {
        self.rows.iter().any(|row| Self::is_winning_line(&row))
            || self
                .columns
                .iter()
                .any(|column| Self::is_winning_line(&column))
    }

    fn unselected(&self, source: &Vec<Vec<Loc>>) -> HashSet<i32> {
        source
            .iter()
            .flat_map(|row| row.iter())
            .filter(|l| !l.m)
            .map(|l| l.n)
            .collect()
    }

    fn score(&self) -> i32 {
        let mut r = self.unselected(&self.rows);

        r.extend(self.unselected(&self.columns));

        self.last_draw * r.iter().fold(0, |a, n| a + n)
    }

    fn finish(&mut self, round: i32, last_draw: i32) {
        self.round = round;
        self.last_draw = last_draw;
    }
    fn is_finished(&self) -> bool {
        self.last_draw != -1
    }
}

fn draws() -> Vec<i32> {
    return vec![
        31, 88, 35, 24, 46, 48, 95, 42, 18, 43, 71, 32, 92, 62, 97, 63, 50, 2, 60, 58, 74, 66, 15,
        87, 57, 34, 14, 3, 54, 93, 75, 22, 45, 10, 56, 12, 83, 30, 8, 76, 1, 78, 82, 39, 98, 37,
        19, 26, 81, 64, 55, 41, 16, 4, 72, 5, 52, 80, 84, 67, 21, 86, 23, 91, 0, 68, 36, 13, 44,
        20, 69, 40, 90, 96, 27, 77, 38, 49, 94, 47, 9, 65, 28, 59, 79, 6, 29, 61, 53, 11, 17, 73,
        99, 25, 89, 51, 7, 33, 85, 70,
    ];
}

fn boards() -> Vec<Board> {
    let lines = fs::read_to_string("days/day4/input.txt").unwrap();

    lines
        .split("\n\n")
        .map(|c| {
            c.split_whitespace()
                .map(|t| t.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(|n| Board::new(&n))
        .collect()
}

fn play_boards(draws: &Vec<i32>, boards: &mut Vec<Board>) {
    for (round, &draw) in draws.iter().enumerate() {
        for board in boards.iter_mut() {
            if !board.is_finished() {
                if board.draw(draw) {
                    board.finish(round as i32 + 1, draw);
                }
            }
        }

        if boards.iter().all(|b| b.is_finished()) {
            break;
        }
    }
}

pub fn part1() {
    let mut boards = boards();

    play_boards(&draws(), &mut boards);

    boards.sort_by(|a, b| a.round.cmp(&b.round));

    print!(
        "Day 4 part 1 : Board won in round {} with score {}\n",
        boards[0].round,
        boards[0].score()
    );
}

pub fn part2() {
    let mut boards = boards();

    play_boards(&draws(), &mut boards);

    boards.sort_by(|a, b| b.round.cmp(&a.round));

    print!(
        "Day 4 part 2 : Last board complete in round {} with score {}\n",
        boards[0].round,
        boards[0].score()
    );
}

pub fn run() {
    part1();
    part2();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let draws = vec![
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1,
        ];

        let mut board = Board::new(&vec![
            14, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5, 2, 0, 12, 3,
            7,
        ]);

        for (round, &draw) in draws.iter().enumerate() {
            if board.draw(draw) {
                board.finish(round as i32 + 1, draw);
                assert_eq!(draw, 24);
                assert_eq!(round + 1, 12);
                assert_eq!(board.score(), 4512);

                break;
            }
        }
    }
}
