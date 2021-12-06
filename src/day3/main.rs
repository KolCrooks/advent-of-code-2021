use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = File::open("data/day3.txt").unwrap();
    let buf_reader = BufReader::new(file);
    let mut bits = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut total = 0;
    for line in buf_reader.lines() {
        total += 1;
        let l = line.unwrap();

        for (i, c) in l.chars().enumerate() {
            if c == '1' {
                bits[i] += 1;
            }
        }
    }
    let mut num1: u32 = 0;
    let mut num2: u32 = 0;
    for i in 0..12usize {
        num1 <<= 1;
        num2 <<= 1;
        if bits[i] > total / 2 {
            num1 += 1;
        } else {
            num2 += 1;
        }
    }

    println!("{}, {}, {}", num1, num2, num1 * num2);
}

fn part2() {
    let file = File::open("data/day3.txt").unwrap();
    let buf_reader = BufReader::new(file);

    let lines: Vec<String> = buf_reader.lines().map(|l| l.unwrap()).collect();

    let mut cp = lines.clone();
    let mut i = 0;
    while cp.len() > 1 {
        let mut bit = 0;
        for l in &cp {
            if l.chars().nth(i).unwrap() == '1' {
                bit += 1;
            }
        }

        bit = (bit as f32 >= (cp.len() as f32 / 2f32)) as usize;

        cp = cp
            .into_iter()
            .filter(|c| c.get(i..(i + 1)).unwrap().parse::<usize>().unwrap() == bit)
            .collect();
        i += 1;
    }

    let mut num1 = 0;
    for i in cp[0].chars() {
        num1 <<= 1;
        num1 += i.to_digit(10).unwrap();
    }

    let mut cp = lines;
    let mut i = 0;
    while cp.len() > 1 {
        let mut bit = 0;
        for l in &cp {
            if l.chars().nth(i).unwrap() == '1' {
                bit += 1;
            }
        }

        bit = (bit as f32 >= (cp.len() as f32 / 2f32)) as usize;

        cp = cp
            .into_iter()
            .filter(|c| c.get(i..(i + 1)).unwrap().parse::<usize>().unwrap() != bit)
            .collect();
        i += 1;
    }

    let mut num2 = 0;

    for i in cp[0].chars() {
        num2 <<= 1;
        num2 += i.to_digit(10).unwrap();
    }
    println!("{}, {}, {}", num1, num2, num1 * num2);
}
