use std::fs;

fn main() {
    let contents = fs::read_to_string("ex2.input").expect("Something went wrong reading the file");
    let mut array: Vec<usize> = contents
        .split(",")
        .map(|v| v.parse().expect("wrong number"))
        .collect();
    let mut run = true;
    let mut offset: usize = 0;
    array[1] = 12;
    array[2] = 2;
    while run {
        run = compute(&mut array, offset);
        offset = offset + 4;
    }

    println!("{:?}", array);
}

fn compute(input: &mut Vec<usize>, offset: usize) -> bool {
    let op_code = input[offset];
    match op_code {
        1 => {
            let arg1_idx = input[offset + 1];
            let arg2_idx = input[offset + 2];
            let result_idx = input[offset + 3];
            input[result_idx] = input[arg1_idx] + input[arg2_idx];
        },
        2 => {
            let arg1_idx = input[offset + 1];
            let arg2_idx = input[offset + 2];
            let result_idx = input[offset + 3];
            input[result_idx] = input[arg1_idx] * input[arg2_idx];
        },
        99 => return false,
        _ => return false,
    }
    return true;
}

