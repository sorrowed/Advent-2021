use std::collections::HashMap;

use common::{extends, Coordinate};
use pathfinding::prelude::astar;

fn successors(
    p: &Coordinate<i64>,
    map: &HashMap<Coordinate<i64>, u32>,
    limits: (i64, i64),
    topleft: &Coordinate<i64>,
    bottomright: &Coordinate<i64>,
) -> Vec<(Coordinate<i64>, u32)> {
    let mut successors = vec![];

    let (base_width, base_height) = (bottomright.x + 1, bottomright.y + 1);

    let (y_mul, y_offs) = (p.y / base_height, p.y % base_height);
    for x in [-1, 1] {
        let x = p.x + x;

        let (x_mul, x_offs) = (x / base_width, x % base_width);

        if x >= topleft.x && x_mul < limits.0 && y_mul < limits.1 {
            if let Some((_, n)) = map.get_key_value(&Coordinate::new(x_offs, y_offs, 0)) {
                // We need to check in the map using mod math, but add the neigbors in normal scale/coordinates
                successors.push((
                    Coordinate::new(x, p.y, p.z),
                    calculate_cost(x_mul, y_mul, n),
                ));
            }
        }
    }

    let (x_mul, x_offs) = (p.x / base_width, p.x % base_width);
    for y in [-1, 1] {
        let y = p.y + y;

        let (y_mul, y_offs) = (y / base_height, y % base_height);

        if y >= topleft.y && x_mul < limits.0 && y_mul < limits.1 {
            if let Some((_, n)) = map.get_key_value(&Coordinate::new(x_offs, y_offs, 0)) {
                successors.push((
                    Coordinate::new(p.x, y, p.z),
                    calculate_cost(x_mul, y_mul, n),
                ));
            }
        }
    }

    successors
}

fn calculate_cost(y_mul: i64, x_mul: i64, n: &u32) -> u32 {
    let mut n: u32 = n + x_mul as u32 + y_mul as u32;
    if n > 9 {
        n = 1 + n % 10;
    }
    n
}

fn part1() {
    let input = common::import("days/day15/input.txt");
    let b = input.iter().map(|s| s.as_str()).collect::<Vec<_>>();
    let map = common::enumerate_xy(&b, &|_, _, c| c.to_digit(10).unwrap());
    let (top_left, bottom_right) = extends(map.keys().copied());

    let path = astar(
        &top_left,
        |p: &Coordinate<i64>| successors(p, &map, (1, 1), &top_left, &bottom_right),
        |p: &Coordinate<i64>| p.crow(&bottom_right) as u32,
        |p: &Coordinate<i64>| *p == bottom_right,
    );

    let cost = if let Some(p) = path {
        p.1
    } else {
        panic!("No path found");
    };

    assert_eq!(cost, 390);

    println!("Day 15 part 1 : Cost of path is {}", cost);
}

fn part2() {
    let input = common::import("days/day15/input.txt");
    let b = input.iter().map(|s| s.as_str()).collect::<Vec<_>>();
    let map = common::enumerate_xy(&b, &|_, _, c| c.to_digit(10).unwrap());

    let (top_left, bottom_right) = extends(map.keys().copied());
    let goal = Coordinate::new(
        5 * (bottom_right.x + 1) - 1,
        5 * (bottom_right.y + 1) - 1,
        bottom_right.z,
    );

    let heuristic = |p: &Coordinate<i64>| p.crow(&goal) as u32;
    let is_at_goal = |p: &Coordinate<i64>| *p == Coordinate::new(goal.x, goal.y, 0);

    let path = astar(
        &top_left,
        |p: &Coordinate<i64>| successors(p, &map, (5, 5), &top_left, &bottom_right),
        heuristic,
        is_at_goal,
    );

    let cost = if let Some(p) = path {
        p.1
    } else {
        panic!("No path found");
    };

    assert_eq!(cost, 2814);

    println!("Day 15 part 1 : Cost of path is {}", cost);
}

pub fn run() {
    part1();
    part2();
}

#[cfg(test)]
mod tests {
    use common::{extends, Coordinate};
    use pathfinding::prelude::astar;

    use crate::successors;

    static TEST_INPUT1: &[&str] = &[
        "1163751742",
        "1381373672",
        "2136511328",
        "3694931569",
        "7463417111",
        "1319128137",
        "1359912421",
        "3125421639",
        "1293138521",
        "2311944581",
    ];

    #[test]
    fn part1_test1() {
        let map = common::enumerate_xy(TEST_INPUT1, &|_, _, c| c.to_digit(10).unwrap());

        let (top_left, bottom_right) = extends(map.keys().copied());

        let heuristic = |p: &Coordinate<i64>| p.crow(&bottom_right) as u32;
        let is_at_goal = |p: &Coordinate<i64>| *p == bottom_right;

        let path = astar(
            &top_left,
            |p: &Coordinate<i64>| successors(p, &map, (1, 1), &top_left, &bottom_right),
            heuristic,
            is_at_goal,
        );

        let cost = if let Some(p) = path {
            p.1
        } else {
            panic!("No path found");
        };

        assert_eq!(cost, 40);
    }

    static TEST_INPUT2: &[&str] = &[
        "11637517422274862853338597396444961841755517295286",
        "13813736722492484783351359589446246169155735727126",
        "21365113283247622439435873354154698446526571955763",
        "36949315694715142671582625378269373648937148475914",
        "74634171118574528222968563933317967414442817852555",
        "13191281372421239248353234135946434524615754563572",
        "13599124212461123532357223464346833457545794456865",
        "31254216394236532741534764385264587549637569865174",
        "12931385212314249632342535174345364628545647573965",
        "23119445813422155692453326671356443778246755488935",
        "22748628533385973964449618417555172952866628316397",
        "24924847833513595894462461691557357271266846838237",
        "32476224394358733541546984465265719557637682166874",
        "47151426715826253782693736489371484759148259586125",
        "85745282229685639333179674144428178525553928963666",
        "24212392483532341359464345246157545635726865674683",
        "24611235323572234643468334575457944568656815567976",
        "42365327415347643852645875496375698651748671976285",
        "23142496323425351743453646285456475739656758684176",
        "34221556924533266713564437782467554889357866599146",
        "33859739644496184175551729528666283163977739427418",
        "35135958944624616915573572712668468382377957949348",
        "43587335415469844652657195576376821668748793277985",
        "58262537826937364893714847591482595861259361697236",
        "96856393331796741444281785255539289636664139174777",
        "35323413594643452461575456357268656746837976785794",
        "35722346434683345754579445686568155679767926678187",
        "53476438526458754963756986517486719762859782187396",
        "34253517434536462854564757396567586841767869795287",
        "45332667135644377824675548893578665991468977611257",
        "44961841755517295286662831639777394274188841538529",
        "46246169155735727126684683823779579493488168151459",
        "54698446526571955763768216687487932779859814388196",
        "69373648937148475914825958612593616972361472718347",
        "17967414442817852555392896366641391747775241285888",
        "46434524615754563572686567468379767857948187896815",
        "46833457545794456865681556797679266781878137789298",
        "64587549637569865174867197628597821873961893298417",
        "45364628545647573965675868417678697952878971816398",
        "56443778246755488935786659914689776112579188722368",
        "55172952866628316397773942741888415385299952649631",
        "57357271266846838237795794934881681514599279262561",
        "65719557637682166874879327798598143881961925499217",
        "71484759148259586125936169723614727183472583829458",
        "28178525553928963666413917477752412858886352396999",
        "57545635726865674683797678579481878968159298917926",
        "57944568656815567976792667818781377892989248891319",
        "75698651748671976285978218739618932984172914319528",
        "56475739656758684176786979528789718163989182927419",
        "67554889357866599146897761125791887223681299833479",
    ];

    #[test]
    fn part2_test1() {
        let map = common::enumerate_xy(TEST_INPUT2, &|_, _, c| c.to_digit(10).unwrap());

        let (top_left, bottom_right) = extends(map.keys().copied());

        let heuristic = |p: &Coordinate<i64>| p.crow(&bottom_right) as u32;
        let is_at_goal =
            |p: &Coordinate<i64>| *p == Coordinate::new(bottom_right.x, bottom_right.y, 0);

        let path = astar(
            &top_left,
            |p: &Coordinate<i64>| successors(p, &map, (1, 1), &top_left, &bottom_right),
            heuristic,
            is_at_goal,
        );

        let cost = if let Some(p) = path {
            p.1
        } else {
            panic!("No path found");
        };

        assert_eq!(cost, 315);
    }

    #[test]
    fn part2_test2() {
        let map = common::enumerate_xy(TEST_INPUT1, &|_, _, c| c.to_digit(10).unwrap());

        let (top_left, bottom_right) = extends(map.keys().copied());
        let goal = Coordinate::new(
            5 * (bottom_right.x + 1) - 1,
            5 * (bottom_right.y + 1) - 1,
            bottom_right.z,
        );

        let heuristic = |p: &Coordinate<i64>| p.crow(&goal) as u32;
        let is_at_goal = |p: &Coordinate<i64>| *p == Coordinate::new(goal.x, goal.y, 0);

        let path = astar(
            &top_left,
            |p: &Coordinate<i64>| successors(p, &map, (5, 5), &top_left, &bottom_right),
            heuristic,
            is_at_goal,
        );

        let cost = if let Some(p) = path {
            p.1
        } else {
            panic!("No path found");
        };

        assert_eq!(cost, 315);
    }
}
