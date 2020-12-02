// use aoc;
use std::fs;

fn check_valid_1(i_key: &str) -> u32{
    let info = i_key.split(" ").collect::<Vec<_>>();
    let bounds = info[0].split("-").collect::<Vec<_>>();
    let lb: u32 = bounds[0].parse().unwrap();
    let hb: u32 = bounds[1].parse().unwrap();
    let key = info[1].chars().nth(0).unwrap();
    let password = info[2];
    let c: u32 = password.matches(key).count() as u32;
    if hb >= c && c >= lb {
        return 1;
    }
    return 0;
}


fn check_valid_2(i_key: &str) -> u32{
    let info = i_key.split(" ").collect::<Vec<_>>();
    
    let mut positions = info[0].split("-");
    let pos_1: usize = positions.next().unwrap().parse().unwrap();
    let pos_2: usize = positions.next().unwrap().parse().unwrap();

    let key = info[1].chars().nth(0).unwrap();
    let password = info[2];
    let corrects = ((password.chars().nth(pos_1 - 1).unwrap() == key) as u32) + ((password.chars().nth(pos_2 - 1).unwrap() == key) as u32);
    if corrects == 1 {
        return 1;
    }
    return 0;
}

fn main() {
    let filename = "data/input.txt";
    // let data = aoc::vector_from_file::<char>(filename).expect("Error getting data");
    let contents = fs::read_to_string(filename).unwrap();
    let data = contents.trim().split("\r\n").collect::<Vec<_>>();

    let num_valid_1 : u32 = data.iter().map(|x| check_valid_1(x)).sum();
    let num_valid_2 : u32 = data.iter().map(|x| check_valid_2(x)).sum();

    println!("Part 1: {} valid results", num_valid_1);
    println!("Part 2: {} valid results", num_valid_2);
}
