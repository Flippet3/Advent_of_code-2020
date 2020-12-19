use std::fs;
use std::time::Instant;
use std::collections::HashMap;
use regex::Regex;

fn create_reg_ex(rules : &Vec<String>, base_rule : usize, mut rule_hash : HashMap<usize, String>, include_loops : bool) -> (String, HashMap<usize, String>) {
    if rule_hash.contains_key(&base_rule) {
        return (rule_hash[&base_rule].to_string(), rule_hash);
    }
    let mut my_rules;
    let mut my_regex_rules : String = "".to_string();

    if rules[base_rule].contains('|') { // If two rules exist
        my_rules = rules[base_rule][5..].split(" | ").collect::<Vec<_>>();
        my_regex_rules.push('(');
    } else {
        my_rules = Vec::new();
        my_rules.push(&rules[base_rule][5..]);
        if my_rules[0].len() == 1 {
            return (my_rules[0].to_string(), rule_hash);
        }
    }
    for i in 0..my_rules.len() {
        if i > 0 {
            my_regex_rules.push('|');
        }
        let my_seq = my_rules[i].split(" ").collect::<Vec<_>>().iter().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
        if include_loops && base_rule == 11 {
            my_regex_rules.push('(');
            for n in 1..10 {
                for i_s in 0..my_seq.len() {
                    for _ in 0..n {
                        let (regex, rule_h) = create_reg_ex(&rules, my_seq[i_s], rule_hash, include_loops);
                        // let (regex, rule_h) = ("bla", rule_hash);
                        rule_hash = rule_h;
                        my_regex_rules += &(regex);
                    }
                }
                if n != 9 {
                    my_regex_rules.push('|');
                }
            }
            my_regex_rules.push(')');
            // println!("{:}", my_regex_rules);
        } else {
            for i_s in 0..my_seq.len() {
                let (regex, rule_h) = create_reg_ex(&rules, my_seq[i_s], rule_hash, include_loops);
                rule_hash = rule_h;
                my_regex_rules += &(regex);
            }
        }
    }

    if my_rules.len() > 1 {
        my_regex_rules.push(')');
    }
    if include_loops && base_rule == 8 {
        my_regex_rules.push('+');
    }

    rule_hash.insert(base_rule, my_regex_rules.to_string());
    return (my_regex_rules, rule_hash);

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
    let paragraphs = contents.replace("\"", "");
    let paragraphs = paragraphs.split("\r\n\r\n").collect::<Vec<_>>();

    let rules = paragraphs[0].split("\r\n").collect::<Vec<_>>();
    let mut rules = rules.iter().map(|x|
    {let mut new_x : String = "".to_string();
    let it = x.chars().position(|x| x == ':').unwrap();
    for _ in 0..(3-(it as i8)) {
        new_x.push_str("0");
    }
    new_x.push_str(x);
    new_x}
    ).collect::<Vec<_>>();
    rules.sort();
    let check_list = paragraphs[1].split("\r\n").collect::<Vec<_>>();
    
    let now = Instant::now();
    
    let (regex_str, _rule_hash) = create_reg_ex(&rules, 0, HashMap::new(), false);
    let regex_str = "^".to_string() + &regex_str + "$";
    let re = Regex::new(&regex_str.to_string()).unwrap();
    let answer1 = check_list.iter().map(|x| re.is_match(x) as usize).sum::<usize>();
    let (regex_str, _rule_hash) = create_reg_ex(&rules, 0, HashMap::new(), true);
    let regex_str = "^".to_string() + &regex_str + "$";
    let re = Regex::new(&regex_str.to_string()).unwrap();
    let answer2 = check_list.iter().map(|x| re.is_match(x) as usize).sum::<usize>();

    println!("{}", now.elapsed().as_secs_f64());

    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);
}
