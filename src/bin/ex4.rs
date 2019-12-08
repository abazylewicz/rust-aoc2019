use std::fs;

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
    let duplicate: u32 = 2;
    let counts: Vec<u32> = Vec::new();
    vec.iter().fold((counts, &0u32), |(mut sums, last), next| {
        if last == next {
            let last_idx = sums.len() - 1;
            sums[last_idx] = sums[last_idx] + 1
        } else {
            sums.push(1)
        }
        (sums, next)
    }).0.contains(&duplicate)
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
