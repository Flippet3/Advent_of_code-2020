use std::fs;

fn check_if_possible (sub_int_list : Vec<usize>, number_needed : usize) -> bool{
    for i in 0..sub_int_list.len() {
        for j in i+1..sub_int_list.len() {
            if sub_int_list[i] + sub_int_list[j] == number_needed {
                return true;
            }
        }
    }
    return false;
}

fn find_first_error (int_list : Vec<usize>, n : usize) -> usize {
    let mut i_n = n;
    let result;
    loop {
        let sub_list = int_list[i_n-n..i_n].to_vec();
        if check_if_possible(sub_list, int_list[i_n]) {
            i_n += 1;
            continue;
        }
        result = int_list[i_n];
        break;
    }
    return result;
}

fn find_streak (int_list : Vec<usize>, sum_required : usize) -> usize {
    let mut sum = int_list[0] + int_list[1];
    let mut i_s = 0;
    let mut i_e = 1;
    loop {
        if sum == sum_required {
            let sub_list = &int_list[i_s..i_e+1];
            let max = sub_list.iter().max().unwrap();
            let min = sub_list.iter().min().unwrap();
            return min+max;
        }
        if sum < sum_required {
            sum += int_list[i_e+1];
            i_e += 1;
        } else {
            sum -= int_list[i_s];
            i_s += 1;
        }
    }
}

fn main() {
    let test = false;
    let filename;
    let n;
    if test {
        filename = "data/input test.txt";
        n = 5;
    } else {
        filename = "data/input.txt";
        n = 25;
    }
    let contents = fs::read_to_string(filename).expect("Could not read the file");
    let data = contents.split("\r\n").collect::<Vec<_>>();
    let data = data.iter().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
    let answer1 = find_first_error(data.to_vec(), n);
    let answer2 = find_streak(data, answer1);
    
    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);
}
