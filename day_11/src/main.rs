use std::fs;
use std::time::Instant;

fn update_seats (layout : &Vec<Vec<bool>>, seats_taken : Vec<Vec<bool>>, seat_style_one : bool) -> (bool, Vec<Vec<bool>>) {
    let mut changed = false;
    let n = layout[0].len();
    let m = layout.len();
    let mut new_seats_taken = seats_taken.to_vec();
    let dirs = vec![[-1,-1],
                    [-1,0],
                    [-1,1],
                    [0,-1],
                    [0,1],
                    [1,-1],
                    [1,0],
                    [1,1]];
    let max_neighbours = if seat_style_one {4} else {5};
    for i_m in 0..m {
        for i_n in 0..n {
            if !layout[i_m][i_n] {
                continue;
            }
            let mut vacinity_taken = 0;
            for dir in &dirs {
                let mut it = 0;
                loop {
                    it += 1;
                    let new_m = i_m as i8 + it * dir[0];
                    let new_n = i_n as i8 + it * dir[1];
                    if new_m < 0 || new_n < 0 || new_m >= m as i8 || new_n >= n as i8 {
                        break;
                    }
                    let new_m = new_m as usize;
                    let new_n = new_n as usize;
                    if !seat_style_one && !layout[new_m][new_n] {
                        continue;
                    }
                    vacinity_taken += seats_taken[new_m][new_n] as usize;
                    break;
                }
            }
            let this_seat = seats_taken[i_m][i_n];
            if this_seat == false {
                if vacinity_taken == 0 {
                    new_seats_taken[i_m][i_n] = true;
                    changed = true;
                }
            } else {
                if vacinity_taken >= max_neighbours {
                    new_seats_taken[i_m][i_n] = false;
                    changed = true;
                }
            }
        }
    }
    return (changed, new_seats_taken);
}

fn resolve_seats(layout : &Vec<Vec<bool>>, seat_style_one : bool) -> Vec<Vec<bool>> {
    let n = layout[0].len();
    let m = layout.len();
    let mut seats_taken = vec![vec![false; n]; m];
    let final_seats;
    loop {
        let (changed, seats_taken_r) = update_seats(layout, seats_taken, seat_style_one);
        seats_taken = seats_taken_r;
        if !changed {
            final_seats = seats_taken;
            break;
        }
    }
    return final_seats;
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
    let layout = data.iter().map(|x| x.chars().collect::<Vec<_>>().iter().map(|y| *y == 'L').collect::<Vec<_>>()).collect::<Vec<_>>();

    // println!("{:?}", layout);

    let now = Instant::now();
    let answer1 = resolve_seats(&layout, true).iter().map(|x| x.iter().map(|y| (*y==true) as usize).sum::<usize>()).sum::<usize>();
    let answer2 = resolve_seats(&layout, false).iter().map(|x| x.iter().map(|y| (*y==true) as usize).sum::<usize>()).sum::<usize>();
    println!("{}", now.elapsed().as_secs_f64());

    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);
}
