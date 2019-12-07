use std::fs;
use std::ops::RangeInclusive;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Orientation {
    Vertical,
    Horizontal,
}


#[derive(Debug, Copy, Clone)]
enum Direction {
    D,
    R,
    L,
    U,
}

impl Direction {
    fn orientation(&self) -> Orientation {
        match self {
            Direction::D => Orientation::Vertical,
            Direction::U => Orientation::Vertical,
            Direction::R => Orientation::Horizontal,
            Direction::L => Orientation::Horizontal,
        }
    }
}

#[derive(Debug, Copy, Clone)]
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
        let (x, y) = self.next_coords();
        Position { x, y, next_move }
    }

    fn next_coords(&self) -> (i32, i32) {
        match self.next_move.direction {
            Direction::D => (self.x, self.y - self.next_move.length),
            Direction::U => (self.x, self.y + self.next_move.length),
            Direction::R => (self.x + self.next_move.length, self.y),
            Direction::L => (self.x - self.next_move.length, self.y),
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
    let inputs: Vec<Vec<Position>> = inputs.iter().map(|input| {
        let init_val: Vec<Position> = Vec::new();
        input.iter().fold(init_val, |mut positions, a_move| {
            positions.push(positions.last()
                .map(|pos| pos.next(*a_move))
                .unwrap_or(Position { x: 0, y: 0, next_move: *a_move }));
            positions
        })
    }).collect();
    let mut crossings: Vec<(i32, i32)> = Vec::new();
    for position in &inputs[0] {
        for another_position in &inputs[1] {
            for coords in crossing(position, another_position) {
                crossings.push(coords)
            }
        }
    }
    crossings.sort_by(|c1, c2| { (c1.0.abs() + c1.1.abs()).cmp(&(c2.0.abs() + c2.1.abs())) });
    crossings.retain(|c| { c.0 != 0 || c.1 != 0});
    let mut lenghts = crossings.iter().map(|c| {
        roadTo(c, &inputs[0]) + roadTo(c, &inputs[1])
    }).collect::<Vec<i32>>();
    lenghts.sort();
    let result = crossings[0];
    println!("{:?}", crossings);
    println!("{:?} : {}", result, result.0.abs()+ result.1.abs());
    println!("{:?}", lenghts);
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

fn crossing(pos1: &Position, pos2: &Position) -> Option<(i32, i32)> {
    let first_orientation = pos1.next_move.direction.orientation();
    let second_orientation = pos2.next_move.direction.orientation();
    if first_orientation != second_orientation {
        //println!("Possible crossing:{:?} -> {:?} and {:?} -> {:?}", (pos1.x, pos1.y), pos1.next_coords(), (pos2.x, pos2.y), pos2.next_coords());
        match first_orientation {
            Orientation::Vertical => {
                positions_crossing(pos1, pos2)
            }
            Orientation::Horizontal => {
                positions_crossing(pos2, pos1)
            }
        }
    } else {
        None
    }
}

fn roadTo(coord:&(i32,i32), positions: &Vec<Position>) -> i32 {
    let mut sum = 0;
    for position in positions {
        let (next_x,next_y) = position.next_coords();
        if position.x == coord.0 && inc_range(position.y, next_y).contains(&coord.1) {
            return sum + (position.y.abs() - coord.1.abs()).abs()
        }
        else if position.y == coord.1 && inc_range(position.x, next_x).contains(&coord.0) {
            return sum + (position.x.abs() - coord.0.abs()).abs()
        } else {
            sum +=position.next_move.length
        }
    }
    panic!("Road to {:?} not found!", coord)
}

fn positions_crossing(vertical: &Position, horizontal: &Position) -> Option<(i32, i32)> {
    let horizontal_range = inc_range(horizontal.x, horizontal.next_coords().0);
    let vertical_range = inc_range(vertical.y, vertical.next_coords().1);
    //println!("horizontal: {:?}, vertical: {:?} and x:{:?} y:{:?}", horizontal_range, vertical_range, vertical.x, horizontal.y);
    if horizontal_range.contains(&vertical.x) && vertical_range.contains(&horizontal.y) {
        Some((vertical.x, horizontal.y))
    } else {
        None
    }
}

fn inc_range(i1: i32, i2: i32) -> RangeInclusive<i32> {
    if i1 <= i2 {
        i1..=i2
    } else {
        i2..=i1
    }
}




