use std::fs;
use std::time::Instant;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

fn speak_game (numbers : Vec<usize>, stop_after : usize) -> usize {
    let mut number_hash = HashMap::with_capacity(10000);
    let mut last_num = numbers[numbers.len()-1];
    let stop_after = stop_after - 1;
    for i in 0..numbers.len()-1 {
        number_hash.insert(numbers[i], i);
    }
    for it in numbers.len()-1..stop_after {
        match number_hash.entry(last_num) {
            Entry::Vacant(e) => {e.insert(it); last_num = 0;},
            Entry::Occupied(mut e) => {let prev_it = e.get().clone(); e.insert(it); last_num = it - prev_it;}
        }
    }
    return last_num;
}

fn main() {
    let test = false;
    let filename;
    if test {
        filename = "data/input test.txt";
    } else {
        filename = "data/input.txt";
    }
    let contents = fs::read_to_string(filename).expect("Could not read the file");
    let data = contents.trim().split(",").collect::<Vec<_>>().iter().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();

    let answer1 = speak_game(data.to_vec(), 2020);
    let now = Instant::now();
    let answer2 = speak_game(data, 30000000);
    println!("{}", now.elapsed().as_secs_f64());

    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);
}
