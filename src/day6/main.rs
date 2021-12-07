use std::{
    collections::{HashMap, LinkedList},
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

    let init_fish: Vec<u32> = raw_fish
        .split(",")
        .map(|x| x.trim().parse().unwrap())
        .collect();

    let mut fish: HashMap<u32, u64> = HashMap::new();

    for f in init_fish {
        fish.entry(f).and_modify(|f| *f += 1).or_insert(1);
    }

    for d in 0..cnt {
        // println!("Day: {}", d);

        let mut temp: HashMap<u32, u64> = HashMap::new();

        for f in fish {
            if f.0 == 0 {
                temp.entry(6).and_modify(|v| *v += f.1).or_insert(f.1);
                temp.insert(8, f.1);
            } else {
                temp.entry(f.0 - 1).and_modify(|v| *v += f.1).or_insert(f.1);
            }
        }
        fish = temp;
    }
    println!("{}", fish.values().fold(0, |acc, v| acc + v));
}
