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
    length: i32,
}

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
    next_move: Move,
}

impl Position {
    fn next(&self, next_move: Move) -> Position {
        match self.next_move.direction {
            Direction::D => Position { x: self.x, y: self.y - self.next_move.length, next_move },
            Direction::U => Position { x: self.x, y: self.y + self.next_move.length, next_move },
            Direction::R => Position { x: self.x + self.next_move.length, y: self.y, next_move },
            Direction::L => Position { x: self.x - self.next_move.length, y: self.y, next_move },
        }
    }
}


fn main() {
    let content = fs::read_to_string("ex3.input").expect("Something went wrong reading the file");
    let inputs = content.lines().map(|line| line
        .split(",")
        .map(|word| Move {
            direction: parse_direction(&word[0..1]),
            length: word[1..].parse().expect("wrong number"),
        })
        .collect::<Vec<Move>>()).collect::<Vec<Vec<Move>>>();
    let first_input = inputs.get(0).expect("to be there");
    let init_val: Vec<Position> = Vec::new();
    first_input.iter().fold(init_val, |mut positions, a_move| {
        positions.push(positions.last()
            .map(|pos| pos.next(*a_move))
            .unwrap_or(Position { x: 0, y: 0, next_move: *a_move }));
        positions
    });
    println!("{:?}", inputs[0]);
}

fn parse_direction(direction: &str) -> Direction {
    match direction {
        "D" => Direction::D,
        "U" => Direction::U,
        "R" => Direction::R,
        "L" => Direction::L,
        _ => panic!("Invalid direction")
    }
}




