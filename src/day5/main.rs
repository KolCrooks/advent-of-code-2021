#![feature(drain_filter)]

use std::{
    borrow::BorrowMut,
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(std::cmp::PartialEq, std::cmp::Eq, std::hash::Hash)]
struct Point {
    pub x: u32,
    pub y: u32,
}

fn main() {
    part1();
    // part2();
}

fn part1() {
    let file = File::open("data/day5-test.txt").unwrap();
    let buf_reader = BufReader::new(file);
    let mut points: HashMap<Point, u32> = HashMap::new();
    for _l in buf_reader.lines() {
        let line = _l.unwrap();

        let pnts: Vec<Vec<u32>> = line
            .split("->")
            .map(|l| {
                l.trim()
                    .split(',')
                    .map(|p| p.trim().parse::<u32>().unwrap())
                    .collect()
            })
            .collect();

        if pnts[0][0] != pnts[1][0] {
            let p = if pnts[0][0] < pnts[1][0] {
                (pnts[0][0], pnts[1][0])
            } else {
                (pnts[1][0], pnts[0][0])
            };

            for i in p.0..p.1 + 1 {
                points
                    .entry(Point {
                        x: pnts[0][1],
                        y: i,
                    })
                    .and_modify(|v| *v += 1)
                    .or_insert(1);
            }
        } else {
            let p = if pnts[0][1] < pnts[1][1] {
                (pnts[0][1], pnts[1][1])
            } else {
                (pnts[1][1], pnts[0][1])
            };
            for i in p.0..p.1 + 1 {
                points
                    .entry(Point {
                        y: pnts[0][1],
                        x: i,
                    })
                    .and_modify(|v| *v += 1)
                    .or_insert(1);
            }
        }
    }

    // *points.entry((pnts[1][0], pnts[1][1])).or_insert(0) += 1;

    let mut count = 0;
    for i in 0..10 {
        for j in 0..10 {
            if let Some(c) = points.get(&Point { x: i, y: j }) {
                count += 1;
                print!("{}", c);
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!("{}", count);
}
fn part2() {
    let file = File::open("data/day5.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    for _l in buf_reader.lines() {
        let line = _l.unwrap();
    }
}
