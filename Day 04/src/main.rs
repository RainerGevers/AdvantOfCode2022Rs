use std::{fs::File, io::{BufReader, BufRead}};

#[derive(Debug)]
struct NumRange {
    low: i32,
    high: i32,
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let ranges = read_ranges();

    let mut overlap = 0;

    for (low, high) in ranges {
        if low.low >= high.low && low.high <= high.high {
            overlap += 1;
            continue;
        }

        if high.low >= low.low && high.high <= low.high {
            overlap += 1;
            continue;
        }
    }

    println!("Complete overlapping ranges: {}", overlap);
}

fn part2() {
    let ranges = read_ranges();

    let mut overlap = 0;

    for (low, high) in ranges {
        if low.low <= high.high && low.high >= high.low   {
            overlap += 1;
            continue;
        }

        if high.low <= low.high && high.high >= low.low {
            overlap += 1;
            continue;
        }
    }

    println!("Overlapping ranges: {}", overlap);
}

fn read_ranges() -> Vec<(NumRange, NumRange)> {
    let file = File::open("files/data.txt").unwrap();
    let reader = BufReader::new(file);

    let mut ranges: Vec<(NumRange, NumRange)> = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        let components: Vec<&str> = line.split(',').collect();

        let part1: Vec<&str> = components[0].split('-').collect();
        let part2: Vec<&str> = components[1].split('-').collect();

        let p1 = NumRange {
            low: part1[0].parse().unwrap(),
            high: part1[1].parse().unwrap()
        };

        let p2 = NumRange {
            low: part2[0].parse().unwrap(),
            high: part2[1].parse().unwrap()
        };

        ranges.push((p1, p2))
    }

    ranges
}