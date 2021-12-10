use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = File::open("data/day9.txt").unwrap();
    let buf_reader = BufReader::new(file);
    let map = buf_reader
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u32)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut cnt = 0u32;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            let val = map[i][j];
            let mut around: Vec<u32> = Vec::new();
            if i > 0 {
                around.push(map[i - 1][j]);
            }
            if i < map.len() - 1 {
                around.push(map[i + 1][j]);
            }
            if j > 0 {
                around.push(map[i][j - 1]);
            }
            if j < map[i].len() - 1 {
                around.push(map[i][j + 1]);
            }
            if around.into_iter().all(|x| x > val) {
                cnt += val + 1;
            }
        }
    }

    println!("{}", cnt);
}

fn part2() {
    let file = File::open("data/day9.txt").unwrap();
    let buf_reader = BufReader::new(file);
    let map = buf_reader
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u32)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut lows = Vec::new();
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            let val = map[i][j];
            let mut around: Vec<u32> = Vec::new();
            if i > 0 {
                around.push(map[i - 1][j]);
            }
            if i < map.len() - 1 {
                around.push(map[i + 1][j]);
            }
            if j > 0 {
                around.push(map[i][j - 1]);
            }
            if j < map[i].len() - 1 {
                around.push(map[i][j + 1]);
            }
            if around.into_iter().all(|x| x > val) {
                lows.push((i, j));
            }
        }
    }
    let mut basins = Vec::new();
    for low in lows {
        let mut worklist = vec![low];
        let mut seen = HashSet::new();
        while !worklist.is_empty() {
            let (i, j) = worklist.pop().unwrap();
            seen.insert((i, j));

            if i > 0 && map[i - 1][j] != 9 && !seen.contains(&(i - 1, j)) {
                worklist.push((i - 1, j));
            }
            if i < map.len() - 1 && map[i + 1][j] != 9 && !seen.contains(&(i + 1, j)) {
                worklist.push((i + 1, j));
            }
            if j > 0 && map[i][j - 1] != 9 && !seen.contains(&(i, j - 1)) {
                worklist.push((i, j - 1));
            }
            if j < map[i].len() - 1 && map[i][j + 1] != 9 && !seen.contains(&(i, j + 1)) {
                worklist.push((i, j + 1));
            }
        }
        basins.push(seen.len());
    }

    basins.sort_unstable();
    let v = basins[basins.len() - 1] * basins[basins.len() - 2] * basins[basins.len() - 3];
    println!("{}", v);
}
