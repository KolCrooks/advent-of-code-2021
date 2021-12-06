#![feature(drain_filter)]

use std::{
    borrow::BorrowMut,
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    // part1();
    part2();
}
const DIM: usize = 1000;

fn part1() {
    let file = File::open("data/day5.txt").unwrap();
    let buf_reader = BufReader::new(file);

    let mut points_out = [[0u8; DIM]; DIM];
    for _l in buf_reader.lines() {
        let line = _l.unwrap();

        let parsed_points: Vec<Vec<u32>> = line
            .split("->")
            .map(|l| {
                l.trim()
                    .split(',')
                    .map(|p| p.trim().parse::<u32>().unwrap())
                    .collect()
            })
            .collect();

        if parsed_points[0][1] == parsed_points[1][1] {
            let order = if parsed_points[0][0] < parsed_points[1][0] {
                (parsed_points[0][0], parsed_points[1][0])
            } else {
                (parsed_points[1][0], parsed_points[0][0])
            };

            for i in order.0..(order.1 + 1) {
                points_out[parsed_points[0][1] as usize][i as usize] += 1;
            }
        } else if parsed_points[0][0] == parsed_points[1][0] {
            let order = if parsed_points[0][1] < parsed_points[1][1] {
                (parsed_points[0][1], parsed_points[1][1])
            } else {
                (parsed_points[1][1], parsed_points[0][1])
            };
            for i in order.0..(order.1 + 1) {
                points_out[i as usize][parsed_points[0][0] as usize] += 1;
            }
        }
    }

    // *points.entry((pnts[1][0], pnts[1][1])).or_insert(0) += 1;

    let mut count = 0;
    for i in 0..DIM {
        for j in 0..DIM {
            if points_out[i][j] > 1 {
                count += 1;
                // print!("{}", points_out[i][j]);
            }
        }
        // println!();
    }

    println!("{}", count);
}

fn part2() {
    let file = File::open("data/day5.txt").unwrap();
    let buf_reader = BufReader::new(file);

    let mut points_out = [[0u8; DIM]; DIM];
    for _l in buf_reader.lines() {
        let line = _l.unwrap();

        let parsed_points: Vec<Vec<u32>> = line
            .split("->")
            .map(|l| {
                l.trim()
                    .split(',')
                    .map(|p| p.trim().parse::<u32>().unwrap())
                    .collect()
            })
            .collect();

        if parsed_points[0][1] == parsed_points[1][1] {
            let order = if parsed_points[0][0] < parsed_points[1][0] {
                (parsed_points[0][0], parsed_points[1][0])
            } else {
                (parsed_points[1][0], parsed_points[0][0])
            };

            for i in order.0..(order.1 + 1) {
                points_out[parsed_points[0][1] as usize][i as usize] += 1;
            }
        } else if parsed_points[0][0] == parsed_points[1][0] {
            let order = if parsed_points[0][1] < parsed_points[1][1] {
                (parsed_points[0][1], parsed_points[1][1])
            } else {
                (parsed_points[1][1], parsed_points[0][1])
            };
            for i in order.0..(order.1 + 1) {
                points_out[i as usize][parsed_points[0][0] as usize] += 1;
            }
        } else {
            let order = if parsed_points[0][1] < parsed_points[1][1] {
                (parsed_points[0].clone(), parsed_points[1].clone())
            } else {
                (parsed_points[1].clone(), parsed_points[0].clone())
            };

            let dir: i32 = if order.0[0] < order.1[0] { 1 } else { -1 };

            for i in 0..(order.1[1] - order.0[1] + 1) {
                points_out[(order.0[1] + i) as usize]
                    [(order.0[0] as i32 + dir * i as i32) as usize] += 1;
            }
        }
    }

    // *points.entry((pnts[1][0], pnts[1][1])).or_insert(0) += 1;

    let mut count = 0;
    for i in 0..DIM {
        for j in 0..DIM {
            if points_out[i][j] > 1 {
                count += 1;
                // print!("{}", points_out[i][j]);
            } else {
                // print!(".");
            }
        }
        // println!();
    }

    println!("{}", count);
}
