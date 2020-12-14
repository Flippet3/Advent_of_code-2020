use std::fs;
use std::time::Instant;
use std::collections::HashMap;

fn write_to_memory(commands : Vec<&str>) -> usize{
    let mut mask = "0";
    let base : i64 = 2;
    let mut mem_entries = HashMap::new();
    for i in 0..commands.len() {
        let command = commands[i].split(" = ").collect::<Vec<_>>();
        let mut command_iter = command.iter();
        let command_type = *command_iter.next().unwrap();
        let command_value = *command_iter.next().unwrap();
        let mut mem_value = 0;
        let entry;
        if command_type[0..3] == *"mas" {
            mask = command_value;
            continue;
        }
        let command_num = &command_value.parse::<i64>().unwrap();
        let s = format!("{:036b}", command_num);
        let string_represent = s.clone();
        let string_represent_pointer = string_represent.as_str();
        entry = &command_type[4..command_type.len()-1];

        for i in 0..36 {
            if mask[i..i+1] == *"X" {
                mem_value += if &string_represent_pointer[i..i+1] == "1" {base.pow((35-i) as u32)} else {0};
            } else {
                mem_value += if &mask[i..i+1] == "1" {base.pow((35-i) as u32)} else {0};
            }
        }
        if mem_entries.contains_key(entry) {
            mem_entries.remove(entry);
        }
        mem_entries.insert(entry, mem_value);
    }
    let mut total_sum = 0;
    for (_, val) in mem_entries {
        total_sum += val;
    }
    return total_sum as usize;
}

fn write_to_memory_2(commands : Vec<&str>) -> usize{
    let mut mask = "0";
    let mut mem_entries = HashMap::new();
    for i in 0..commands.len() {
        let command = commands[i].split(" = ").collect::<Vec<_>>();
        let mut command_iter = command.iter();
        let command_type = *command_iter.next().unwrap();
        let command_value = *command_iter.next().unwrap();
        if command_type[0..3] == *"mas" {
            mask = command_value;
            continue;
        }
        let mem_value = command_value.parse::<i64>().unwrap();
        let entry = command_type[4..command_type.len()-1].parse::<i64>().unwrap();
        let s = format!("{:036b}", entry);
        let string_represent = s.clone();
        let string_represent_pointer = string_represent.as_str();

        let char_vec_vec : Vec<Vec<char>>= vec![vec![]];

        fn add_to_list (mask : &str, string_represent_pointer : &str, mut char_vec : Vec<Vec<char>>, mut i : usize) -> Vec<Vec<char>> {
            if i == 36 {
                return char_vec;
            }
            if mask[i..i+1] == *"0" {
                char_vec[0].push(string_represent_pointer.chars().nth(i).unwrap());
                i += 1;
            } else if mask[i..i+1] == *"1" {
                char_vec[0].push('1');
                i += 1;
            } else {
                i += 1;
                let mut char_vec_1 = char_vec.to_vec();
                char_vec_1[0].push('0');
                let mut vec1 = add_to_list(mask, string_represent_pointer, char_vec_1, i);
                let mut char_vec_2 = char_vec.to_vec();
                char_vec_2[0].push('1');
                let mut vec2 = add_to_list(mask, string_represent_pointer, char_vec_2, i);
                vec1.append(&mut vec2);
                return vec1;
            }
            return add_to_list(mask, string_represent_pointer, char_vec, i);
        }

        let char_vec_vec = add_to_list(mask, string_represent_pointer, char_vec_vec, 0);
        let entries = char_vec_vec.iter().map(|x| x.into_iter().collect::<String>()).collect::<Vec<_>>();
        let num_entries = entries.iter().map(|x| usize::from_str_radix(x, 2).unwrap()).collect::<Vec<_>>();

        for entry in num_entries {
            if mem_entries.contains_key(&entry) {
                mem_entries.remove(&entry);
            }
            mem_entries.insert(entry, mem_value);
        }
    }
    let mut total_sum = 0;
    for (_, val) in mem_entries {
        // println!("entry: {:?}", entr);
        // println!("value: {:?}", val);
        total_sum += val;
    }
    return total_sum as usize;
}

fn main() {
    let test = false;
    let filename;
    if test {
        filename = "data/input test2.txt";
    } else {
        filename = "data/input.txt";
    }
    let contents = fs::read_to_string(filename).expect("Could not read the file");
    let data = contents.trim().split("\r\n").collect::<Vec<_>>();
    
    let now = Instant::now();
    let answer1 = write_to_memory(data.to_vec());
    let answer2 = write_to_memory_2(data);
    println!("{}", now.elapsed().as_secs_f64());

    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);
}
