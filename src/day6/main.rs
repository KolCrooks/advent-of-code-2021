use std::{
    collections::LinkedList,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    parts(80);
    parts(256);
}

fn parts(cnt: u32) {
    let file = File::open("data/day6.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut raw_fish = String::new();
    buf_reader.read_line(&mut raw_fish).unwrap();

    let mut fish: Vec<u32> = raw_fish
        .split(",")
        .map(|x| x.trim().parse().unwrap())
        .collect();

    for d in 0..cnt {
        println!("Day: {}", d);
        for i in 0..fish.len() {
            if fish[i] == 0 {
                fish[i] = 6;
                fish.push(8);
            } else {
                fish[i] -= 1;
            }
        }
    }
    println!("{}", fish.len());
}

fn part2() {}
