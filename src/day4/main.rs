#![feature(drain_filter)]

use std::{
    borrow::BorrowMut,
    fs::File,
    io::{BufRead, BufReader},
};

struct BingoBoard {
    squares: [[u32; 5]; 5],
    marked_squares: [[bool; 5]; 5],
}

impl BingoBoard {
    fn new(rows: Vec<String>) -> Self {
        let mut squares = [[0; 5]; 5];
        for (r, row) in rows.into_iter().enumerate() {
            for i in 0..4 {
                let s = row.get((3 * i)..3 * (i + 1)).unwrap();

                squares[r][i] = s.trim().parse::<u32>().unwrap();
            }
            squares[r][4] = row.get(4 * 3..).unwrap().trim().parse::<u32>().unwrap();
        }
        Self {
            squares,
            marked_squares: [[false; 5]; 5],
        }
    }
    fn mark(&mut self, num: u32) -> Option<u32> {
        let mut pos_attempt = None;
        'outer: for (r, row) in self.squares.iter().enumerate() {
            for (c, sq) in row.iter().enumerate() {
                if *sq == num {
                    pos_attempt = Some((r, c));
                    break 'outer;
                }
            }
        }
        let pos = pos_attempt?;

        self.marked_squares[pos.0][pos.1] = true;

        // check row
        if !self.marked_squares[pos.0].into_iter().any(|i| !i) {
            return Some(self.unmarked_sum());
        }

        // check col
        if !self.marked_squares.into_iter().any(|i| !i[pos.1]) {
            return Some(self.unmarked_sum());
        }

        None
    }

    fn unmarked_sum(&self) -> u32 {
        let mut sum = 0;
        for (i, row) in self.marked_squares.into_iter().enumerate() {
            for (j, c) in row.into_iter().enumerate() {
                if !c {
                    sum += self.squares[i][j];
                }
            }
        }
        sum
    }
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = File::open("data/day4.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut boards: Vec<BingoBoard> = Vec::new();
    let mut nums_str: String = String::new();
    buf_reader.read_line(&mut nums_str).unwrap();

    let nums: Vec<u32> = nums_str
        .split(',')
        .map(|n| n.trim_end().parse::<u32>().unwrap())
        .collect();
    buf_reader.read_line(&mut nums_str).unwrap();
    let mut buff: Vec<String> = Vec::new();
    for raw_line in buf_reader.lines() {
        let line = raw_line.unwrap();
        if line.is_empty() {
            continue;
        }
        buff.push(line);
        if buff.len() == 5 {
            boards.push(BingoBoard::new(buff));
            buff = Vec::new();
        }
    }

    for n in nums {
        for b in boards.iter_mut() {
            if let Some(sum) = b.mark(n) {
                println!("{}, {}, {}", sum * n, sum, n);
                return;
            }
        }
    }
}
fn part2() {
    let file = File::open("data/day4.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut boards: Vec<BingoBoard> = Vec::new();
    let mut nums_str: String = String::new();
    buf_reader.read_line(&mut nums_str).unwrap();

    let nums: Vec<u32> = nums_str
        .split(',')
        .map(|n| n.trim_end().parse::<u32>().unwrap())
        .collect();
    buf_reader.read_line(&mut nums_str).unwrap();
    let mut buff: Vec<String> = Vec::new();
    for raw_line in buf_reader.lines() {
        let line = raw_line.unwrap();
        if line.is_empty() {
            continue;
        }
        buff.push(line);
        if buff.len() == 5 {
            boards.push(BingoBoard::new(buff));
            buff = Vec::new();
        }
    }
    let mut last_win = 0;

    for n in nums {
        boards.drain_filter(|b| {
            if let Some(sum) = b.mark(n) {
                if sum != 0 {
                    last_win = sum * n;
                }
                return true;
            }
            false
        });
    }

    println!("{}", last_win);
}
