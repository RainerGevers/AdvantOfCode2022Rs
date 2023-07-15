use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = File::open("files/p1.txt").unwrap();
    let reader = BufReader::new(file);

    let letter_points = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(idx, c)| (c, idx + 1))
        .collect::<HashMap<char, usize>>();

    let mut points_vec: Vec<usize> = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        let sections = line.split_at(line.len()/2);
        for chara in sections.0.chars() {
            if sections.1.contains(chara) {
                match letter_points.get(&chara) {
                    Some(val) => points_vec.push(*val),
                    None => panic!("Could not match chara")
                }
                break
            }
        }
    }

    println!("{:?}", points_vec.iter().sum::<usize>());
}

fn part2() {
    let file = File::open("files/p1.txt").unwrap();
    let reader = BufReader::new(file);

    let letter_points = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(idx, c)| (c, idx + 1))
        .collect::<HashMap<char, usize>>();

    let mut counter = 0;
    let mut vec: Vec<String> = vec![];
    let mut results: Vec<usize> = vec![];

    for line in reader.lines() {
        let line = line.unwrap();
        vec.push(line);
        counter += 1;
        if counter == 3 {
            counter = 0;
            if let Some(common) = find_common_char(&vec) {
                if let Some(&val) = letter_points.get(&common) {
                    results.push(val);
                } else {
                    panic!("Could not match character");
                }
            }
            vec.clear();
        }

    }

    println!("{:?}", results.iter().sum::<usize>());
}

fn find_common_char(vec: &Vec<String>) -> Option<char> {
    let mut chars: Vec<Vec<char>> = vec.iter().map(|s| s.chars().collect()).collect();
    for chars_set in chars.iter_mut() {
        chars_set.sort_unstable();
    }
    for &char in chars[0].iter() {
        if chars[1].binary_search(&char).is_ok() && chars[2].binary_search(&char).is_ok() {
            return Some(char);
        }
    }
    None
}