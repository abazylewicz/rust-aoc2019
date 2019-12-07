use std::fs;
use std::ops::RangeInclusive;

fn main() {
    let range = 246540..787419;
    println!("Elligible passwords: {}", range.filter(|pass| is_elligible(pass)).count())
}

fn is_elligible(pass: &i32) -> bool {
    const RADIX: u32 = 10;
    let pass = pass.to_string();
    let x = pass.chars().map(|char| char.to_digit(RADIX).expect("Should be number!")).collect::<Vec<u32>>();
    is_sorted(&x) && has_duplicate(&x)
}

fn has_duplicate(vec: &Vec<u32>) -> bool {
    let mut prev: Option<&u32> = None;
    for next in vec {
        match prev {
            Some(prev_val) if next == prev_val =>{
                    return true;
            }
            _ => prev = Some(next),
        }
    }
    false
}

fn is_sorted(vec: &Vec<u32>) -> bool {
    let mut prev = &vec[0];
    for next in vec {
        if next < prev {
            return false;
        }
        prev = next;
    }
    true
}
