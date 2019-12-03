use std::fs;

fn main() {
    let contents = fs::read_to_string("1-1.input").expect("Something went wrong reading the file");
    let sum: u32 = contents
        .lines()
        .map(|weight_str| weight_str.parse::<u32>().expect("wrong number"))
        .map(|w| fuel_for(w))
        .sum();
    println!("{}", sum);
}

fn fuel_for(weight: u32) -> u32 {
    let fuel_needed = (weight / 3).checked_sub(2).unwrap_or(0);
    return if fuel_needed == 0 {
        0
    } else {
        fuel_needed + fuel_for(fuel_needed)
    };
}
