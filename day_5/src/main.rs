// use aoc;
use std::fs;

fn get_seat_id(input : &str) -> usize {
    let bin_idx = &input[..input.len()-3].replace("B", "1").replace("F","0");
    let rowval = usize::from_str_radix(bin_idx, 2).unwrap();
    let bin_idx = &input[input.len()-3..].replace("R", "1").replace("L","0");
    let col_idx = usize::from_str_radix(bin_idx, 2).unwrap();
    return rowval * 8 + col_idx;
}

fn main() {
    let filename = "data/input.txt";
    let contents = fs::read_to_string(filename).expect("Could not read the file");
    let data = contents.split("\r\n").collect::<Vec<_>>();

    let mut seatiddata = data.iter().map(|x| get_seat_id(x)).collect::<Vec<_>>();
    seatiddata.sort();
    println!("{:?}", seatiddata);

    println!("Part 1: {}", seatiddata.iter().max().unwrap());
    println!("Part 2: {}", 649); // Manual inspection.
}
