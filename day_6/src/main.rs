use std::fs;

fn get_unique_chars(input : &Vec<char>) -> Vec<char> {
    let mut input_copy = input.to_vec();
    input_copy.sort();
    input_copy.dedup();
    return input_copy;
}

fn count_unique_characters(input : &Vec<char>) -> usize {
    let un_char = get_unique_chars(input);
    return un_char.len();
}

fn count_overlap_characters(input : &Vec<Vec<char>>) -> usize {
    let mut shared_answers = input[0].to_vec();
    let mut j;
    for i in 1..input.len() {
        j=0;
        while j < shared_answers.len() {
            if !input[i].contains(&shared_answers[j]) {
                shared_answers.remove(j);
            } else {
                j += 1;
            }
        }
    }
    return shared_answers.len();
}

fn main() {
    let filename = "data/input.txt";
    let contents = fs::read_to_string(filename).expect("Could not read the file");
    let data = contents.split("\r\n\r\n").collect::<Vec<_>>();
    let data1 = &data.iter().map(|x| x.replace("\r\n", "").chars().collect()).collect::<Vec<_>>();
    let data2: Vec<Vec<Vec<char>>> = data.iter().map(|x| x.split("\r\n").collect::<Vec<_>>().iter().map(|x| x.chars().collect::<Vec<_>>()).collect::<Vec<_>>()).collect::<Vec<_>>();

    let answers_nr : usize = data1.iter().map(|x| count_unique_characters(x)).sum();
    let answer_nr2 : usize = data2.iter().map(|x| count_overlap_characters(x)).sum();
    // seatiddata.sort();

    println!("Part 1: {}", answers_nr);
    println!("Part 2: {}", answer_nr2);
}
