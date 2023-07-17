use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

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
    let ranges = read_ranges("files/data.txt");

    let contains = |low: &NumRange, high: &NumRange| -> bool {
        (low.low >= high.low && low.high <= high.high) || (high.low >= low.low && high.high <= low.high)
    };

    let overlap = ranges
        .iter()
        .filter(|(low, high)| contains(low, high) )
        .count();

    println!("Complete overlapping ranges: {}", overlap);
}

fn part2() {
    let ranges = read_ranges("files/data.txt");

    let overlaps = |low: &NumRange, high: &NumRange| -> bool {
        (low.low <= high.high && low.high >= high.low) || (high.low <= low.high && high.high >= low.low)
    };

    let overlap = ranges
        .iter()
        .filter(|(low, high)| overlaps(low, high))
        .count();

    println!("Overlapping ranges: {}", overlap);
}

fn read_ranges<P: AsRef<Path>>(file_path: P) -> Vec<(NumRange, NumRange)> {
    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| {
            let line = line.expect("Failed to read line");
            let components: Vec<&str> = line.split(',').collect();

            let part1: Vec<&str> = components[0].split('-').collect();
            let part2: Vec<&str> = components[1].split('-').collect();

            let p1 = NumRange {
                low: part1[0].parse().expect("Failed to parse number"),
                high: part1[1].parse().expect("Failed to parse number"),
            };

            let p2 = NumRange {
                low: part2[0].parse().expect("Failed to parse number"),
                high: part2[1].parse().expect("Failed to parse number"),
            };

            (p1, p2)
        })
        .collect()
}