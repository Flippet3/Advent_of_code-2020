use std::fs;
use std::time::Instant;
use std::collections::HashMap;

fn speak_game (numbers : Vec<usize>, stop_after : usize) -> usize {
    let mut number_hash = HashMap::with_capacity(10000);
    let mut last_num = numbers[numbers.len()-1];
    let stop_after = stop_after - 1;
    for i in 0..numbers.len()-1 {
        number_hash.insert(numbers[i], i);
    }
    let mut prev_it;
    for it in numbers.len()-1..stop_after {
        // println!("{:?}", number_hash);
        if number_hash.contains_key(&last_num) {
            prev_it = number_hash[&last_num];
            number_hash.insert(last_num, it);
            last_num = it - prev_it;
        } else {
            number_hash.insert(last_num, it);
            last_num = 0;
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
    let data = contents.trim().split(",").collect::<Vec<_>>();
    let data = data.iter().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();

    let now = Instant::now();
    let answer1 = speak_game(data.to_vec(), 2020);
    let answer2 = speak_game(data, 30000000);
    println!("{}", now.elapsed().as_secs_f64());

    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);
}
