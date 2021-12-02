use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = File::open("data/day2.txt").unwrap();
    let buf_reader = BufReader::new(file);
    let mut x: i32 = 0;
    let mut y: i32 = 0;

    for line in buf_reader.lines() {
        let l = line.unwrap();
        let parts: Vec<&str> = l.split(' ').collect();
        let val = parts[1].parse::<i32>().unwrap();
        match parts[0] {
            "forward" => x += val,
            "up" => y -= val,
            "down" => y += val,
            _ => (),
        };
    }

    println!("pos: ({},{}), x*y={}", x, y, x * y);
}

fn part2() {
    let file = File::open("data/day2.txt").unwrap();
    let buf_reader = BufReader::new(file);
    let mut x: i32 = 0;
    let mut aim: i32 = 0;
    let mut y: i32 = 0;

    for line in buf_reader.lines() {
        let l = line.unwrap();
        let parts: Vec<&str> = l.split(' ').collect();
        let val = parts[1].parse::<i32>().unwrap();
        match parts[0] {
            "forward" => {
                x += val;
                y += aim * val
            }
            "up" => aim -= val,
            "down" => aim += val,
            _ => (),
        };
    }

    println!("pos: ({},{}), x*y={}", x, y, x * y);
}
