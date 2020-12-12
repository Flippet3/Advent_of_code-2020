use std::fs;
use std::time::Instant;

fn get_position_1(instructions : &Vec<&str>) -> (i32, i32) {
    let mut x_pos = 0;
    let mut y_pos = 0;
    let dirs : Vec<Vec<i32>> = vec![vec![1,0],
                                    vec![0,-1],
                                    vec![-1,0],
                                    vec![0,1]];
    let mut dir_i : i32 = 0;
    for i in 0..instructions.len() {
        let instr = instructions[i];
        let key = &instr[0..1];
        let amount = &instr[1..].parse::<i32>().unwrap();
        match key {
            "N" => y_pos += amount,
            "S" => y_pos -= amount,
            "E" => x_pos += amount,
            "W" => x_pos -= amount,
            "F" => {x_pos += dirs[dir_i as usize][0] * amount; y_pos += dirs[dir_i as usize][1] * amount;},
            "R" => dir_i = (dir_i + (*amount /90))%4,
            "L" => dir_i = (4 + dir_i - (*amount /90))%4,
            _ => println!{"No Match found for the key"},
        }
        // println!{"x: {:?}", x_pos};
        // println!{"y: {:?}", y_pos};
    }
    return (x_pos,y_pos);
}


fn get_position_2(instructions : &Vec<&str>) -> (i32, i32) {
    let mut x_pos = 0;
    let mut y_pos = 0;
    let mut x_way = 10;
    let mut y_way = 1;
    for i in 0..instructions.len() {
        let instr = instructions[i];
        let key = &instr[0..1];
        let amount = &instr[1..].parse::<i32>().unwrap();

        match key {
            "N" => y_way += amount,
            "S" => y_way -= amount,
            "E" => x_way += amount,
            "W" => x_way -= amount,
            "F" => {x_pos += x_way * amount; y_pos += y_way * amount;},
            "R" => {if *amount == 90 {let x_st = x_way; x_way = y_way; y_way = -x_st} else if *amount == 180 {x_way = -x_way; y_way = -y_way} else if *amount == 270 {let x_st = x_way; x_way = -y_way; y_way = x_st}},
            "L" => {if *amount == 90 {let x_st = x_way; x_way = -y_way; y_way = x_st} else if *amount == 180 {x_way = -x_way; y_way = -y_way} else if *amount == 270 {let x_st = x_way; x_way = y_way; y_way = -x_st;}},
            _ => println!{"No Match found for the key"},
        }
        // println!{"x: {:?}", x_pos};
        // println!{"y: {:?}", y_pos};
        // println!{"x_way: {:?}", x_way};
        // println!{"y_way: {:?}", y_way};
    }
    return (x_pos,y_pos);
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
    let data = contents.trim().split("\r\n").collect::<Vec<_>>();

    let now = Instant::now();
    let (x_dist, y_dist) = get_position_1(&data);
    let answer1 = x_dist.abs() + y_dist.abs();
    let (x_dist_2, y_dist_2) = get_position_2(&data);
    let answer2 = x_dist_2.abs() + y_dist_2.abs();
    println!("{}", now.elapsed().as_secs_f64());

    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);
}
