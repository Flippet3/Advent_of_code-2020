// use aoc;
use std::fs;


fn check_for_tree_at_pos(data : &Vec<&str>, x : usize, y : usize) -> bool {
    let row = data[y];
    if row.chars().nth(x%row.len()).unwrap() == '#'{
        return true;
    }
    return false;
}
    
fn go_down_with_slope(data : &Vec<&str>, x_slope : usize, y_slope : usize) -> usize {
    let mut nr_trees_met : usize = 0;
    for i in 1..data.len()/y_slope {
        if check_for_tree_at_pos(&data, i*x_slope, i*y_slope) {
            nr_trees_met += 1;
        }
    }
    return nr_trees_met;

}

fn main() {
    let filename = "data/input.txt";
    let contents = fs::read_to_string(filename).expect("Could not read the file");
    let data = contents.trim().split("\r\n").collect::<Vec<_>>();

    let part_1_result = go_down_with_slope(&data, 3, 1);

    let part_2_result = go_down_with_slope(&data, 1, 1) *
                        go_down_with_slope(&data, 3, 1) *
                        go_down_with_slope(&data, 5, 1) *
                        go_down_with_slope(&data, 7, 1) *
                        go_down_with_slope(&data, 1, 2);

    println!("Part 1: {}", part_1_result);
    println!("Part 2: {}", part_2_result);
}
