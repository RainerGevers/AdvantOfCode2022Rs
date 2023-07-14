use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

fn main() {
    let mut elf_index = 0;
    let mut calories: HashMap<i32, i32> = HashMap::new();

    let file = File::open("files/q1.txt").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();

        if line == "" {
            elf_index += 1;
            continue
        }

        let calorie = line.parse::<i32>().unwrap();
        let sum = calories.entry(elf_index).or_insert(0);
        *sum += calorie;
    }

    let max_pair = get_top_n_values(&calories, 1);

    println!("The biggest value in the hashmap is {}", max_pair[0]);

    let max_3_pair: i32 = get_top_n_values(&calories, 3).iter().sum();

    println!("The 3 biggest values in the hashmap is {}", max_3_pair);
}

fn get_top_n_values(hash: &HashMap<i32,i32>, n: i32) -> Vec<i32> {
    let mut values = Vec::new();

    for value in hash.values() {
        values.push(*value);
    }

    values.sort();
    values.reverse();

    values.truncate(n as usize);

    values
}
