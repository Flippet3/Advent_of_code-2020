use std::fs;
use std::time::Instant;
use std::collections::hash_map::Entry;
use std::collections::HashMap;


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

fn create_4d_cubes(input_data : Vec<Vec<bool>>) -> HashMap<(i16, i16, i16, i16), usize> {
    let mut cubes : HashMap<(i16, i16, i16, i16), usize> = HashMap::new();
    for i in 0..input_data.len() {
        for j in 0..input_data[0].len() {
            if input_data[i][j] {
                cubes.insert((i as i16,j as i16,0,0),0);
            }
        }
    }
    return cubes;
}

fn play_conway(mut cubes : HashMap<(i16, i16, i16), usize>, nr_rounds : usize) -> usize {
    for _i in 0..nr_rounds {
        let mut neighbouring_cubes : HashMap<(i16, i16, i16), usize> = HashMap::new();
        for (key, _) in &cubes {
            let (x,y,z) = key;
            for dx in -1..2 {
                for dy in -1..2 {
                    for dz in -1..2 {
                        if dx == 0 && dy == 0 && dz == 0 {
                            continue;
                        }
                        match neighbouring_cubes.entry((x+dx, y+dy, z+dz)) {
                            Entry::Vacant(e) => {e.insert(1);},
                            Entry::Occupied(mut e) => {e.insert(e.get() + 1);}
                        }
                    }
                }
            }
        }
        for (key, value) in neighbouring_cubes {
            match cubes.entry(key) {
                Entry::Vacant(e) => {if value == 3 {e.insert(0);}},
                Entry::Occupied(e) => {if !(value == 2 || value == 3) {e.remove_entry();}}
            }
        }
    }
    return cubes.len();
}

fn play_conway_4d(mut cubes : HashMap<(i16, i16, i16,i16), usize>, nr_rounds : usize) -> usize {
    for _i in 0..nr_rounds {
        // let mut neighbouring_cubes : HashMap<(i16, i16, i16,i16), usize> = HashMap::new();
        let mut neighbouring_cubes = cubes.clone();
        for (key, _) in &cubes {
            let (x,y,z,w) = key;
            for dx in -1..2 {
                for dy in -1..2 {
                    for dz in -1..2 {
                        for dw in -1..2 {
                            if dx == 0 && dy == 0 && dz == 0 && dw == 0 {
                                continue;
                            }
                            match neighbouring_cubes.entry((x+dx, y+dy, z+dz, w+dw)) {
                                Entry::Vacant(e) => {e.insert(1);},
                                Entry::Occupied(mut e) => {e.insert(e.get() + 1);}
                            }
                        }
                    }
                }
            }
        }
        for (key, value) in neighbouring_cubes.clone() {
            match cubes.entry(key) {
                Entry::Vacant(e) => {if value == 3 {e.insert(0);}},
                Entry::Occupied(e) => {if !(value == 2 || value == 3) {e.remove_entry();}}
            }
        }
    }
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
    let cubes = create_cubes(data.to_vec());
    let cubes_4d = create_4d_cubes(data);
    let now = Instant::now();
    let answer1 = play_conway(cubes, 6);
    let answer2 = play_conway_4d(cubes_4d, 6);

    println!("{}", now.elapsed().as_secs_f64());

    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);
}
