use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = File::open("data/day10.txt").unwrap();
    let buf_reader = BufReader::new(file);
    let mut score_table = HashMap::new();
    score_table.insert(')', 3);
    score_table.insert(']', 57);
    score_table.insert('}', 1197);
    score_table.insert('>', 25137);
    let mut score = 0;
    for l in buf_reader.lines() {
        let mut stack = Vec::new();
        let line = l.unwrap();

        for c in line.chars() {
            if c == '(' || c == '[' || c == '{' || c == '<' {
                stack.push(c);
            } else {
                let last = stack.pop().unwrap();
                match (last, c) {
                    ('(', ')') => (),
                    ('[', ']') => (),
                    ('{', '}') => (),
                    ('<', '>') => (),
                    _ => {
                        score += score_table.get(&c).unwrap();
                        break;
                    }
                }
            }
        }
    }
    println!("{}", score);
}

fn part2() {
    let file = File::open("data/day10.txt").unwrap();
    let buf_reader = BufReader::new(file);
    let mut score_table = HashMap::new();
    score_table.insert('(', 1);
    score_table.insert('[', 2);
    score_table.insert('{', 3);
    score_table.insert('<', 4);

    let mut scores = Vec::new();
    'lines: for l in buf_reader.lines() {
        let mut stack = Vec::new();
        let line = l.unwrap();

        for c in line.chars() {
            if c == '(' || c == '[' || c == '{' || c == '<' {
                stack.push(c);
            } else {
                let last = stack.pop().unwrap();
                match (last, c) {
                    ('(', ')') => (),
                    ('[', ']') => (),
                    ('{', '}') => (),
                    ('<', '>') => (),
                    _ => {
                        continue 'lines;
                    }
                }
            }
        }
        // println!("{}", stack.len());
        let v = stack
            .into_iter()
            .rev()
            .fold(0u64, |acc, v| acc * 5 + score_table.get(&v).unwrap());
        if v != 0 {
            scores.push(v);
        }
    }
    scores.sort_unstable();

    println!("{}, {}", scores[scores.len() / 2], scores.len());
}
