use std::{
    collections::{HashMap, LinkedList},
    fs::File,
    io::{BufRead, BufReader},
    ops::RangeBounds,
};

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = File::open("data/day8.txt").unwrap();
    let buf_reader = BufReader::new(file);
    let mut cnt = 0;

    for raw_line in buf_reader.lines() {
        let line = raw_line.unwrap();
        cnt += line
            .split(" | ")
            .nth(1)
            .unwrap()
            .split(' ')
            .fold(0, |acc, v| {
                acc + (v.len() == 2 || v.len() == 3 || v.len() == 4 || v.len() == 7) as u32
            });
    }

    println!("{:?}", cnt);
}

fn part2() {
    let file = File::open("data/day8.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut digits = [0u32; 10];
    let mut cnt = 0;

    for raw_line in buf_reader.lines() {
        let line = raw_line.unwrap();
        let parts = line.split(" | ").collect::<Vec<_>>();
        let segs = parts[0].split(' ').collect::<Vec<_>>();
        let output = parts[1].split(' ').collect::<Vec<_>>();
        let seg_5 = segs.iter().filter(|s| s.len() == 5).collect::<Vec<&&str>>();

        let middle_3 = &*seg_5.iter().fold(seg_5[0].to_string(), |acc, v| {
            acc.chars().filter(|s| v.chars().any(|v| v == *s)).collect()
        });

        let mut translation = [""; 10];
        translation[1] = *segs.iter().find(|s| s.len() == 2).unwrap();
        translation[7] = segs.iter().find(|s| s.len() == 3).unwrap();
        translation[8] = segs.iter().find(|s| s.len() == 7).unwrap();
        translation[4] = segs.iter().find(|s| s.len() == 4).unwrap();

        translation[3] = segs
            .iter()
            .find(|s| s.len() == 5 && is_subset(s, translation[1]) && is_subset(s, middle_3))
            .unwrap();
        translation[9] = segs
            .iter()
            .find(|s| s.len() == 6 && is_subset(s, translation[4]) && is_subset(s, middle_3))
            .unwrap();
        translation[0] = segs
            .iter()
            .find(|s| s.len() == 6 && !is_subset(s, middle_3))
            .unwrap();
        translation[6] = segs
            .iter()
            .find(|s| s.len() == 6 && **s != translation[9] && **s != translation[0])
            .unwrap();
        translation[5] = segs
            .iter()
            .find(|s| s.len() == 5 && **s != translation[3] && is_subset(translation[9], s))
            .unwrap();
        translation[2] = segs
            .iter()
            .find(|s| {
                s.len() == 5
                    && **s != translation[3]
                    && **s != translation[5]
                    && **s != translation[6]
            })
            .unwrap();

        let out = output.iter().fold(String::new(), |acc, v| {
            acc + &translation
                .iter()
                .position(|s| is_subset(s, v) && is_subset(v, s))
                .unwrap()
                .to_string()
        });
        cnt += out.parse::<u32>().unwrap();
    }

    println!("{:?}", cnt);
}

fn is_subset(s1: &str, sub: &str) -> bool {
    sub.chars().all(|c| s1.contains(c))
}
