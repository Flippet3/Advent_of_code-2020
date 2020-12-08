use std::fs;

fn find_acc_after_first_double(input : Vec<&str>) -> (i32, bool) {
    let mut i : i32 = 0;
    let mut acc : i32 = 0;
    let mut i_handeled = Vec::new();
    let mut instruction;
    let mut succes = false;
    loop {
        if i_handeled.contains(&i) {
            break;
        }
        i_handeled.push(i);
        instruction = &input[i as usize][0..3];
        if instruction == "acc" {
            acc = acc + input[i as usize][4..].parse::<i32>().unwrap();
            i += 1;
        } else if instruction == "nop" {
            i += 1;
        } else if instruction == "jmp" {
            i += &input[i as usize][4..].parse::<i32>().unwrap();
        }
        if i as usize == input.len() {
            succes = true;
            break;
        }
    }

    return (acc, succes);
}

fn fix_programme(input : Vec<&str>) -> i32 {
    let mut acc = 0;
    for i in 0..input.len() {
        let mut input_copy = input.to_vec();
        let instruction = &input[i as usize][0..3];
        let value : String;
        if instruction == "acc" {
            continue;
        } else if instruction == "jmp" {
            value = ["nop", &input[i as usize][4..]].join(" ");
            input_copy[i] = value.as_str();
        } else if instruction == "nop" {
            value = ["jmp", &input[i as usize][4..]].join(" ");
            input_copy[i] = value.as_str();
        }
        let (acc_found, succes) = find_acc_after_first_double(input_copy);
        if succes {
            acc = acc_found;
            println!("Succes!!");
            break;
        }
    }
    return acc;
}

fn main() {
    let filename = "data/input.txt";
    let contents = fs::read_to_string(filename).expect("Could not read the file");
    let data = contents.split("\r\n").collect::<Vec<_>>();
    

    let (answer1, _) = find_acc_after_first_double(data.to_vec());
    let answer2 = fix_programme(data);

    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);
}
