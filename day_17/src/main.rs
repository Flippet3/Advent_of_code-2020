use std::fs;
use std::time::Instant;
use std::collections::HashMap;

// #[derive(Debug)]
// struct Cube {
//     neighbours : u8,
//     x : i6,
//     y : i16,
//     z : i16,
// }

fn create_cubes(input_data : Vec<Vec<bool>>) -> HashMap<(i16, i16, i16), usize> {
    let mut cubes : HashMap<(i16, i16, i16), usize> = HashMap::new();
    for i in 0..input_data.len() {
        for j in 0..input_data[0].len() {
            if input_data[i][j] {
                cubes.insert((i as i16,j as i16,0),0);
            }
        }
    }
    return cubes;
}

fn play_conway(mut cubes : HashMap<(i16, i16, i16), usize>, nr_rounds : usize) -> usize {
    for _ in 0..nr_rounds {
        let mut neighbouring_cubes : HashMap<(i16, i16, i16), usize> = HashMap::new();
        for (key, _) in &cubes {
            let (x,y,z) = key;
            for dx in -1..2 {
                for dy in -1..2 {
                    for dz in -1..2 {
                        if dx == 0 && dy == 0 && dz == 0 {
                            continue;
                        }
                        let i_key = (x+dx, y+dy, z+dz);
                        if neighbouring_cubes.contains_key(&i_key) {
                            neighbouring_cubes.insert(i_key, neighbouring_cubes[&i_key] + 1);
                        } else {
                            neighbouring_cubes.insert(i_key, 1);
                        }
                    }
                }
            }

        }
        for (key, value) in neighbouring_cubes {
            if cubes.contains_key(&key) {
                if !(value == 2 || value == 3) {
                    cubes.remove(&key);
                }
            } else {
                if value == 3 {
                    cubes.insert(key, 0);
                }
            }
        }
        for (cube, val) in cubes.clone() {
            if val != 0 {
                cubes.insert(cube, 0);
            }
        }
    }
    // println!("{:?}", cubes);
    return cubes.len();
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
    let data = contents.trim().split("\r\n").map(|x| x.chars().map(|x| x == '#').collect::<Vec<_>>()).collect::<Vec<_>>();
    let cubes = create_cubes(data);
    let answer1 = play_conway(cubes, 3);
    let now = Instant::now();

    println!("{}", now.elapsed().as_secs_f64());

    println!("Part 1: {}", answer1);
    // println!("Part 2: {}", answer2);
}
