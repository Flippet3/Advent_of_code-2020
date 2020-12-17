use std::fs;
use std::time::Instant;
// use std::collections::HashMap;

#[derive(Debug)]
struct Cube {
    neighbours : u8,
    x : i8,
    y : i8,
    z : i8,
}

fn create_cubes(input_data : Vec<Vec<bool>>) -> Vec<Cube> {
    let mut cubes : Vec<Cube> = Vec::new();
    for i in 0..input_data.len() {
        for j in 0..input_data[0].len() {
            if input_data[i][j] {
                cubes.push(Cube{neighbours : 0, x : i as i8, y : j as i8, z : 0})
            }
        }
    }
    return cubes;
}

fn play_conway(cubes : Vec<Cube>, nr_rounds : usize) -> usize {
    for i in 0..nr_rounds {
        let mut neighbouring_cubes : Vec<Cube> = Vec::new();
        for cube in cubes
    }
    return 0;
}



fn main() {
    let test = true;
    let filename;
    if test {
        filename = "data/input test.txt";
    } else {
        filename = "data/input.txt";
    }
    let contents = fs::read_to_string(filename).expect("Could not read the file");
    let data = contents.trim().split("\r\n").map(|x| x.chars().map(|x| x == '#').collect::<Vec<_>>()).collect::<Vec<_>>();
    let cubes = create_cubes(data);
    println!("{:?}", cubes);
    let answer1 = play_conway(cubes, 6);
    let now = Instant::now();

    println!("{}", now.elapsed().as_secs_f64());

    println!("Part 1: {}", answer1);
    // println!("Part 2: {}", answer2);
}
