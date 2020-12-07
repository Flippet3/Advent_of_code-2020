use std::fs;

#[derive(Debug)]
struct Bag {
    this_bag : String,
    this_unpacked : bool,
    contents : Vec<String>,
    numbers : Vec<usize>,
    bags_unpacked : Vec<bool>
}

fn create_bag(construct_str : &str) -> Bag {
    let mut contruct_str_split = construct_str.split(" bag contain ");
    let bagname : &str = contruct_str_split.next().unwrap();
    let contents : &str = contruct_str_split.next().unwrap();
    let contents = contents[..contents.len()-1].split(", ").collect::<Vec<_>>();
    if contents[0] == "no other bag" {
        let bag = Bag{
            this_bag : (*bagname).to_string(),
            this_unpacked: false,
            contents: Vec::new(),
            numbers: Vec::new(),
            bags_unpacked: Vec::new(),
        };
        return bag;
    } else {
        let bag = Bag{
            this_bag: (*bagname).to_string(),
            this_unpacked: false,
            contents: contents.iter().map(|x| x[2..x.len()-4].to_string()).collect::<Vec<_>>(),
            numbers: contents.iter().map(|x| x[0..1].parse().unwrap()).collect::<Vec<_>>(),
            bags_unpacked: vec![false; contents.len()],
        };
        return bag;
    }
}

fn resolve_bags(mut bags : Vec<Bag>) -> Vec<Bag> {
    let mut idx;
    loop {
        let mut still_unpackable_bags = false;
        for i in 0..bags.len() {
            if !bags[i].this_unpacked {
                if bags[i].bags_unpacked.iter().all(|&x| x) {
                    bags[i].this_unpacked = true;
                    still_unpackable_bags = true;
                    for j in 0..bags.len() {
                        if i==j {
                            continue;
                        }
                        if bags[j].contents.contains(&bags[i].this_bag) {
                            idx = bags[j].contents.iter().position(|x| x == &bags[i].this_bag).unwrap();
                            let mut content_to_push = bags[i].contents.to_vec();
                            let numbers_to_push = bags[i].numbers.to_vec();
                            let multi = bags[j].numbers[idx];

                            bags[j].contents.append(&mut content_to_push);
                            bags[j].numbers.append(&mut numbers_to_push.iter().map(|x| x * multi).collect::<Vec<_>>());
                            bags[j].bags_unpacked[idx] = true;
                        }
                    }
                }
            }
        }
        if !still_unpackable_bags {
            break;
        }
    }
    return bags;
}

fn main() {
    let filename = "data/input.txt";
    let contents = fs::read_to_string(filename).expect("Could not read the file");
    let contents = contents.replace("bags", "bag");
    let data = contents.split("\r\n").collect::<Vec<_>>();
    let bags = data.iter().map(|x| create_bag(x)).collect::<Vec<_>>();
    let bags = resolve_bags(bags);
    

    let answers_nr : usize = bags.iter().map(|x| x.contents.contains(&(*"shiny gold").to_string()) as usize).sum();
    let gold_bag_index = bags.iter().position(|x| x.this_bag == (*"shiny gold").to_string()).unwrap();
    let answer_2 : usize = bags[gold_bag_index].numbers.iter().sum();

    // println!("{:?}", bags[gold_bag_index]);

    println!("Part 1: {}", answers_nr);
    println!("Part 2: {}", answer_2);
}
