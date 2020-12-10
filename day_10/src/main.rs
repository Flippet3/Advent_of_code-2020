use std::fs;

fn get_diff_list(sort_list : &Vec<usize>) -> Vec<usize> {
    let mut step_list = vec![];
    for i in 1..sort_list.len() {
        step_list.push(sort_list[i] - sort_list[i-1]);
    }
    return step_list;
}

fn record_adapter_steps(sort_list : &Vec<usize>) -> (usize, usize) {
    let step_list = get_diff_list(sort_list);
    let step_one = step_list.iter().map(|x| (*x == 1) as usize).sum::<usize>();
    let step_three = step_list.iter().map(|x| (*x == 3) as usize).sum::<usize>();
 
    return (step_one, step_three);
}

fn count_connection_ways(sort_list : &Vec<usize>) -> usize {
    let adapter_nr = sort_list.len();
    let mut perm_list = vec![0;adapter_nr];
    perm_list[adapter_nr-1] = 1;
    for i in 1..sort_list.len() {
        let i_sl = sort_list.len() - i-1;
        let mut di = 1;
        let this_val = sort_list[i_sl];
        loop {
            let step = sort_list[i_sl+di] - this_val;
            if step <= 3 {
                perm_list[i_sl] += perm_list[i_sl+di];
                if step == 3 {
                    break;
                }
                di += 1;
                continue;
            }
            break;
        }

    }
 
    return perm_list[0];
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
    let data = contents.split("\r\n").collect::<Vec<_>>();
    let mut data = data.iter().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
    data.push(0);
    data.push(data.iter().max().unwrap() + 3);
    data.sort();

    // println!("{:?}", data);

    let (step_one, step_three) = record_adapter_steps(&data);
    println!("{:?}", step_one);
    println!("{:?}", step_three);
    let answer1 = step_one * step_three;
    let answer2 = count_connection_ways(&data);
    
    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);
}
