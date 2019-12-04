use std::fs;

#[derive(Debug)]
enum Direction {
    D,
    R,
    L,
    U,
}

#[derive(Debug)]
struct Move {
    direction: Direction,
    length: u32,
}

fn main() {
    let content = fs::read_to_string("ex3.input").expect("Something went wrong reading the file");
    let content_lines = content.lines().map(|line| line
        .split(",")
        .map(|word| Move {
            direction: parse_direction(&word[0..1]),
            length: word[1..].parse().expect("wrong number"),
        })
        .collect::<Vec<Move>>()).collect::<Vec<Vec<Move>>>();
    println!("{:?}", content_lines[0]);
}

fn parse_direction(direction: &str) -> Direction {
    match direction {
        "D" => Direction::D,
        "R" => Direction::R,
        "U" => Direction::U,
        "L" => Direction::L,
        _ => panic!("Invalid direction")
    }
}




