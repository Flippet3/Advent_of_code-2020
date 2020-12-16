use std::fs;
use std::time::Instant;
// use std::collections::HashMap;



fn check_ranges(store_ints : &Vec<usize>, value: usize) -> bool {
    for i in 0..store_ints.len()/4 {
        if (value>=store_ints[0+i*4] && value<=store_ints[1+i*4]) || (value>=store_ints[2+i*4] && value<=store_ints[3+i*4]) {
            return true;
        }
    }
    return false;
}

fn check_unvalid_tickets(store_ints : &Vec<usize>, mut tickets : Vec<Vec<usize>>) -> (usize, Vec<Vec<usize>>) {
    let mut sum_of_bad_values = 0;
    let mut valid_ticket;
    let mut it = 0;
    loop {
        valid_ticket = true;
        for j in 0..tickets[it].len() {
            if !check_ranges(store_ints, tickets[it][j]) {
                sum_of_bad_values += tickets[it][j];
                valid_ticket = false;
            }
        }
        if !valid_ticket {
            tickets.remove(it);
        } else {
            it += 1;
        }
        if it == tickets.len() {
            break;
        }
    }
    return (sum_of_bad_values, tickets);
}

fn check_row(store_ints : &Vec<usize>, tickets : Vec<Vec<usize>>) -> Vec<usize> {
    let n = store_ints.len()/4;
    let mut truth_matrix = vec![vec![1 as usize; n]; n];
    for i in 0..tickets.len() {
        for j in 0..n {
            for k in 0..n {
                if truth_matrix[j][k] == 0 {
                    continue;
                }
                let value = tickets[i][j];
                if !((value>=store_ints[0+k*4] && value<=store_ints[1+k*4]) || (value>=store_ints[2+k*4] && value<=store_ints[3+k*4])) {
                    truth_matrix[j][k] = 0;
                }
            }
        }
    }
    let mut encryption = vec![0; n];
    let mut truth_sum = truth_matrix.iter().map(|x| x.iter().sum::<usize>()).collect::<Vec<_>>();
    while truth_sum.iter().sum::<usize>() >= 1 {
        let j = truth_sum.iter().position(|x| *x == 1).unwrap();
        let k = truth_matrix[j].iter().position(|x| *x == 1).unwrap();
        encryption[j] = k;
        for i in 0..truth_matrix.len() {
            truth_matrix[i][k] = 0;
        }
        truth_sum = truth_matrix.iter().map(|x| x.iter().sum::<usize>()).collect::<Vec<_>>();
    }
    return encryption;
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
    let data = contents.trim().split("\r\n\r\n").map(|x| x.split("\r\n").collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut store_ints = vec![];
    
    for i in 0..data[0].len() {
        let data_space_split = data[0][i].split(" ").collect::<Vec<_>>();
        let mut val01 = data_space_split[data_space_split.len()-3].split("-");
        store_ints.push(val01.next().unwrap().parse::<usize>().unwrap());
        store_ints.push(val01.next().unwrap().parse::<usize>().unwrap());
        let mut val23 = data_space_split[data_space_split.len()-1].split("-");
        store_ints.push(val23.next().unwrap().parse::<usize>().unwrap());
        store_ints.push(val23.next().unwrap().parse::<usize>().unwrap());
    }
    let tickets = data[2][1..].iter().map(|comma_str| comma_str.trim().split(",").collect::<Vec<_>>().iter().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>();

    let now = Instant::now();
    let (answer1, tickets) = check_unvalid_tickets(&store_ints, tickets);
    let encryption = check_row(&store_ints, tickets);
    let my_ticket = data[1][1].split(",").collect::<Vec<_>>().iter().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
    let mut answer2 = 0;
    for i in 0..6 {
        let pos = encryption.iter().position(|x| *x == i).unwrap();
        if answer2 == 0 {
            answer2 += my_ticket[pos];
        } else {
            answer2 *= my_ticket[pos];
        }
    }
    println!("{}", now.elapsed().as_secs_f64());

    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);
}
