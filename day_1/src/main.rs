use aoc;

fn sort(input: Vec<i32>) -> Vec<i32> {
    let mut result : Vec<i32> = Vec::new();
    result.clear();
    let mut result = input;
    result.sort();
    return result;
}

fn find_pair(input: &Vec<i32>, goal : i32) -> Vec<i32> {
    let mut begin_i = 0;
    let mut results = Vec::new();
    let mut end_i = input.len()-1;
    let mut n = 1;
    while n < 10000 {
        n += 1;
        if begin_i == end_i {
            break;
        }
        let sum = &input[end_i] + &input[begin_i];
        if sum > goal {
            end_i = end_i - 1;
            continue;
        } else if sum < goal {
            begin_i = begin_i + 1;
            continue;
        } else if sum == goal {
            results.push(input[begin_i]);
            results.push(input[end_i]);
            break;
        }
    }
    return results;
}

fn find_triplet(input: &Vec<i32>, goal : i32) -> Vec<i32> {
    let mut results = Vec::new();
    for i in 0..&input.len()-2 {
        for j in 1..&input.len()-1 {
            for k in 2..&input.len()+0 {
                if &input[i] + &input[j] + &input[k] == goal {
                    results.push(input[i]);
                    results.push(input[j]);
                    results.push(input[k]);
                    println!("woopy");
                    println!("{}, {}, {}", i, j, k);
                    break;
                }
            }
        }
    }
    return results;
}
fn main() {
    let filename = "data/input.txt";
    let data = aoc::vector_from_file::<i32>(filename).expect("Error getting data");
    let goal = 2020;
    let sorteddata = sort(data);
    let pairs = find_pair(&sorteddata, goal);
    let triplets = find_triplet(&sorteddata, goal);
    assert_eq!(&triplets[0] + &triplets[1] + &triplets[2], goal);
    println!("Part 1:");
    println!("Value 1: {}", &pairs[0].to_string());
    println!("Value 2: {}", &pairs[1].to_string());
    println!("Product: {}", &pairs[0]*&pairs[1]);
    println!("Part 2:");
    println!("Value 1: {}", &triplets[0].to_string());
    println!("Value 2: {}", &triplets[1].to_string());
    println!("Value 3: {}", &triplets[2].to_string());
    println!("Product: {}", &triplets[0]*&triplets[1]*&triplets[2]);
    

}

