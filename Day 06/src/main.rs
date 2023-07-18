fn main() {
    part1();
    part2();
}

fn part1() {
    let input = std::fs::read_to_string("files/data.txt").unwrap();

    let result = input
        .as_bytes()
        .windows(4)
        .position(|window| {
            window
                .iter()
                .enumerate()
                .all(|(idx, c)| !window[..idx].contains(c))
        })
        .unwrap()
        + 4;

    println!("P1 Start of sequence: {}", result);
}

fn part2() {
    let input = std::fs::read_to_string("files/data.txt").unwrap();

    let result = input
        .as_bytes()
        .windows(14)
        .position(|window| {
            window
                .iter()
                .enumerate()
                .all(|(idx, c)| !window[..idx].contains(c))
        })
        .unwrap()
        + 14;

    println!("P2 Start of sequence: {}", result);
}
