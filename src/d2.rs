// ################# Part 1 #####################

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::{cmp::Ordering, str::FromStr};

#[derive(PartialEq, Copy, Clone)]
enum Hand {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl PartialOrd for Hand {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<Ordering> {

        // Edge Cases for wrap sci (3) loses to rock (1)
        if self == &Hand::Scissors && other == &Hand::Rock {
            return Some(Ordering::Less)
        }

        // Edge case for wrap Rock (1) beats sci (3)
        if self == &Hand::Rock && other == &Hand::Scissors {
            return Some(Ordering::Greater)
        }

        // Numerical compare e.g. Paper(2) beats Rock(1)
        return Some((*self as u8).cmp(&(*other as u8)))
    }
}

impl FromStr for Hand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Hand::Rock),
            "B" | "Y" => Ok(Hand::Paper),
            "C" | "Z" => Ok(Hand::Scissors),
            _ => Err("Not a known move".to_string()),
        }
    }
}


fn main() {
    let file = File::open("files/q2.txt").unwrap();
    let reader = BufReader::new(file);

    let mut points: i32 = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        let moves: Vec<&str> = line.split(" ").collect();

        let opponent_move = moves[0].parse::<Hand>().unwrap();
        let our_move = moves[1].parse::<Hand>().unwrap();

        match opponent_move.partial_cmp(&our_move) {
            Some(Ordering::Equal) => {points += 3 + our_move as i32},
            Some(Ordering::Greater) => {points += our_move as i32},
            Some(Ordering::Less) => {points += 6 + our_move as i32},
            _ => panic!("Issue with compare")
        }
    }

    println!("{}", points);

}

// P2
// fn main() {
//     let file = File::open("files/q2.txt").unwrap();
//     let reader = BufReader::new(file);
//
//     let mut points: i32 = 0;
//
//     for line in reader.lines() {
//         let line = line.unwrap();
//
//         let moves: Vec<&str> = line.split(" ").collect();
//
//         let opponent_move = moves[0].parse::<Hand>().unwrap();
//
//         match moves[1] {
//             "X" => {
//                 let my_hand = match opponent_move {
//                     Hand::Rock => {Hand::Scissors},
//                     Hand::Paper => {Hand::Rock},
//                     Hand::Scissors => {Hand::Paper}
//                 };
//                 points += my_hand as i32
//
//             },
//             "Y" => {
//                 let my_hand = opponent_move.clone();
//                 points += 3 + my_hand as i32
//             },
//             "Z" => {
//                 let my_hand = match opponent_move {
//                     Hand::Rock => {Hand::Paper},
//                     Hand::Paper => {Hand::Scissors},
//                     Hand::Scissors => {Hand::Rock}
//                 };
//                 points += 6 + my_hand as i32
//             },
//             _ => {panic!("Unknown condition")}
//         }
//     }
//
//     println!("{}", points);
//
// }