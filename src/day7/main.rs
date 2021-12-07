use std::{
    collections::{HashMap, LinkedList},
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = File::open("data/day7.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut raw_crabs = String::new();
    buf_reader.read_line(&mut raw_crabs).unwrap();

    let crabs: Vec<u32> = raw_crabs
        .split(",")
        .map(|x| x.trim().parse().unwrap())
        .collect();

    let mut min_diff = u32::MAX;
    let min = *crabs.iter().min().unwrap();
    let max = *crabs.iter().max().unwrap();

    for i in min..(max + 1) {
        let mut sum = 0;
        for c in crabs.iter() {
            sum += (*c as i32 - i as i32).abs() as u32;
        }
        if sum < min_diff {
            min_diff = sum;
        }
    }
    println!("{}", min_diff);
}

fn part2() {
    let file = File::open("data/day7.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut raw_crabs = String::new();
    buf_reader.read_line(&mut raw_crabs).unwrap();

    let crabs: Vec<u32> = raw_crabs
        .split(",")
        .map(|x| x.trim().parse().unwrap())
        .collect();

    let mut min_diff = u32::MAX;
    let min = *crabs.iter().min().unwrap();
    let max = *crabs.iter().max().unwrap();

    for i in min..(max + 1) {
        let mut sum = 0;
        for c in crabs.iter() {
            let n = (*c as i32 - i as i32).abs() as u32;
            sum += n * (n + 1) / 2;
        }
        if sum < min_diff {
            min_diff = sum;
        }
    }
    println!("{}", min_diff);
}
