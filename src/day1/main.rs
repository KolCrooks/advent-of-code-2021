use std::{
    collections::LinkedList,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = File::open("data/day1.txt").unwrap();
    let buf_reader = BufReader::new(file);
    let mut prev = 0;
    let mut increase = -1;

    for line in buf_reader.lines() {
        let l = line.unwrap();
        let new = l.parse::<u32>().unwrap();
        if new > prev {
            increase += 1;
        }
        prev = new
    }

    println!("increase: {}", increase)
}

fn part2() {
    let file = File::open("data/day1.txt").unwrap();
    let buf_reader = BufReader::new(file);
    let mut increase = 0;
    let mut window: LinkedList<u32> = LinkedList::new();

    for line in buf_reader.lines() {
        let l = line.unwrap();
        let new = l.parse::<u32>().unwrap();
        window.push_back(new);
        if window.len() != 4 {
            continue;
        }
        if window.front().unwrap() < &new {
            increase += 1;
        }
        window.pop_front();
    }

    println!("increase: {}", increase)
}
