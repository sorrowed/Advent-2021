use std::collections::HashSet;

use common::dfs_all;

#[derive(PartialEq, Debug, Clone, Eq, Hash)]
struct Cave {
    pub name: String,
}

#[derive(PartialEq, Debug, Clone, Eq, Hash)]
struct Edge {
    pub from: Cave,
    pub to: Cave,
}

impl Cave {
    pub fn is_small(&self) -> bool {
        self.name.chars().all(|c| c.is_ascii_lowercase())
    }

    pub fn start() -> Cave {
        Cave {
            name: "start".to_string(),
        }
    }

    pub fn end() -> Cave {
        Cave {
            name: "start".to_string(),
        }
    }

    pub fn is_start(&self) -> bool {
        self.name == *"start"
    }
    pub fn is_end(&self) -> bool {
        self.name == *"end"
    }
}

fn parse_edge(line: &str) -> Edge {
    let (from, to) = line
        .trim()
        .split_once('-')
        .expect("invalid edge: expected exactly one '-'");

    Edge {
        from: Cave {
            name: from.to_string(),
        },
        to: Cave {
            name: to.to_string(),
        },
    }
}

fn paths_from_edges<H>(edges: &[Edge], cond: &H) -> Vec<Vec<Cave>>
where
    H: Fn(&Cave, &Vec<Cave>) -> bool,
{
    let mut visited = vec![];
    let mut paths = vec![];

    dfs_all(
        &Cave::start(),
        &|current| current.is_end(),
        &|current, visited| {
            //print!("{} --> ", current.name);

            let n = edges
                .iter()
                .filter_map(|edge| {
                    if current == &edge.from {
                        if cond(&edge.to, visited) {
                            Some(edge.to.clone())
                        } else {
                            None
                        }
                    } else if current == &edge.to {
                        if cond(&edge.from, visited) {
                            Some(edge.from.clone())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                //.inspect(|cave| print!("{} ", cave.name))
                .collect::<Vec<_>>();

            //println!();

            n
        },
        &mut visited,
        &mut paths,
    );

    paths
}

fn small_caves(edges: &[Edge]) -> HashSet<Cave> {
    edges
        .iter()
        .flat_map(|edge| {
            let mut r = vec![];

            if edge.from.is_small() {
                r.push(edge.from.clone());
            }
            if edge.to.is_small() {
                r.push(edge.to.clone());
            }
            r
        })
        .collect::<HashSet<_>>()
}

fn is_valid_target_cave_part1(cave: &Cave, visited: &[Cave]) -> bool {
    let c = visited.iter().filter(|c| c == &cave).count();

    !cave.is_start() && (cave.is_end() || !cave.is_small() || c == 0)
}

fn is_valid_target_cave_part2(cave: &Cave, visited: &[Cave], twice: &Cave) -> bool {
    let c = visited.iter().filter(|c| c == &cave).count();

    !cave.is_start() && (cave.is_end() || !cave.is_small() || c == 0 || c == 1 && cave == twice)
}

static INPUT: &[&str] = &[
    "bm-XY", "ol-JS", "bm-im", "RD-ol", "bm-QI", "JS-ja", "im-gq", "end-im", "ja-ol", "JS-gq",
    "bm-AF", "RD-start", "RD-ja", "start-ol", "cj-bm", "start-JS", "AF-ol", "end-QI", "QI-gq",
    "ja-gq", "end-AF", "im-QI", "bm-gq", "ja-QI", "gq-RD",
];

fn part1() {
    let edges = INPUT
        .iter()
        .map(|line| parse_edge(line))
        .collect::<Vec<_>>();

    let solutions = paths_from_edges(&edges, &|cave, visited| {
        is_valid_target_cave_part1(cave, visited)
    });
    assert_eq!(solutions.len(), 3887);

    println!("Day 12 part 1 : Number of paths is {}", solutions.len());
}

fn part2() {
    let edges = INPUT
        .iter()
        .map(|line| parse_edge(line))
        .collect::<Vec<_>>();

    let solutions = small_caves(&edges)
        .iter()
        .flat_map(|twice| {
            paths_from_edges(&edges, &|cave, visited| {
                is_valid_target_cave_part2(cave, visited, twice)
            })
        })
        .collect::<HashSet<_>>();

    assert_eq!(solutions.len(), 104834);

    println!("Day 12 part 2 : Number of paths is {}", solutions.len());
}

pub fn run() {
    part1();
    part2();
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{
        is_valid_target_cave_part1, is_valid_target_cave_part2, parse_edge, paths_from_edges,
        small_caves, Cave,
    };

    static TEST_INPUT1: &[&str] = &["start-A", "start-b", "A-c", "A-b", "b-d", "A-end", "b-end"];
    #[test]
    fn part1_test1() {
        let moves = TEST_INPUT1
            .iter()
            .map(|line| parse_edge(line))
            .collect::<Vec<_>>();

        assert_eq!(moves.len(), 7);
        assert!(moves[0].from.is_start());
        assert!(moves[0].from.is_small());
        assert!(!moves[0].to.is_small());
        assert_eq!(moves[0].to, moves[2].from);
        assert!(moves[5].to.is_small());
        assert!(moves[5].to.is_end());
        assert!(moves[6].to.is_end());
    }

    #[test]
    fn part1_test2() {
        let edges = TEST_INPUT1
            .iter()
            .map(|line| parse_edge(line))
            .collect::<Vec<_>>();

        let solutions = paths_from_edges(&edges, &|cave, visited| {
            is_valid_target_cave_part1(cave, visited)
        });

        println!("{:?}", solutions);

        assert_eq!(solutions.len(), 10);
    }

    static TEST_INPUT2: &[&str] = &[
        "dc-end", "HN-start", "start-kj", "dc-start", "dc-HN", "LN-dc", "HN-end", "kj-sa", "kj-HN",
        "kj-dc",
    ];

    #[test]
    fn part1_test3() {
        let edges = TEST_INPUT2
            .iter()
            .map(|line| parse_edge(line))
            .collect::<Vec<_>>();

        let solutions = paths_from_edges(&edges, &|cave, visited| {
            is_valid_target_cave_part1(cave, visited)
        });

        assert_eq!(solutions.len(), 19);
    }

    static TEST_INPUT3: &[&str] = &[
        "fs-end", "he-DX", "fs-he", "start-DX", "pj-DX", "end-zg", "zg-sl", "zg-pj", "pj-he",
        "RW-he", "fs-DX", "pj-RW", "zg-RW", "start-pj", "he-WI", "zg-he", "pj-fs", "start-RW",
    ];

    #[test]
    fn part1_test4() {
        let edges = TEST_INPUT3
            .iter()
            .map(|line| parse_edge(line))
            .collect::<Vec<_>>();

        let solutions = paths_from_edges(&edges, &|cave, visited| {
            is_valid_target_cave_part1(cave, visited)
        });

        assert_eq!(solutions.len(), 226);
    }

    #[test]
    fn part2_test1() {
        let edges = TEST_INPUT1
            .iter()
            .map(|line| parse_edge(line))
            .collect::<Vec<_>>();

        let solutions = small_caves(&edges)
            .iter()
            .flat_map(|twice| {
                paths_from_edges(&edges, &|cave, visited| {
                    is_valid_target_cave_part2(cave, visited, twice)
                })
            })
            .collect::<HashSet<_>>();

        println!("{:?}", solutions);

        assert_eq!(solutions.len(), 36);
    }
}
