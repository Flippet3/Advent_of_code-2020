// use aoc;
use std::fs;
use hex;

fn check_7_field_presence(input : &Vec<&str>) -> bool {
    let mut keys = input.iter().map(|x| &x[..3]).collect::<Vec<_>>();
    keys.retain(|x| *x != "cid");
    return keys.len() == 7;
}

fn check_correct_fields_present(input : &Vec<&str>) -> bool {
    let keys = input.iter().map(|x| &x[..3]).collect::<Vec<_>>();
    let mut newkeys = keys.to_vec();
    newkeys.retain(|x| *x != "cid");
    if newkeys.len() != 7 {
        return false;
    }
    //byr
    let byr = &input[keys.iter().position(|x| *x == "byr").unwrap()][4..];
    let byr : usize = byr.parse().expect("couldn't parse");
    if byr < 1920 || byr > 2002 {
        // println!("byr incorrect: {0}", byr);
        return false;
    }
    //iyr
    let iyr : usize = input[keys.iter().position(|x| *x == "iyr").unwrap()][4..].parse().expect("couldn't parse");
    if iyr < 2010 || iyr > 2020 {
        // println!("iyr incorrect: {0}", iyr);
        return false;
    }
    //eyr
    let eyr : usize = input[keys.iter().position(|x| *x == "eyr").unwrap()][4..].parse().expect("couldn't parse");
    if eyr < 2020 || eyr > 2030 {
        // println!("eyr incorrect: {0}", eyr);
        return false;
    }
    //hgt
    let hgt = &input[keys.iter().position(|x| *x == "hgt").unwrap()][4..];
    let hgt_id = &hgt[hgt.len()-2..];
    let hgt_val = hgt[..hgt.len()-2].parse();
    let hgt_val : usize = match hgt_val {
        Ok(val) => val,
        Err(_) => return false
    };
    if hgt_id == "cm" {
        if hgt_val < 150 || hgt_val > 193 {
            return false;
        }
    } else if hgt_id == "in" {
        if hgt_val < 59 || hgt_val >  76 {
            return false;
        }
    } else {
        return false;
    }

    //hcl
    let hcl = &input[keys.iter().position(|x| *x == "hcl").unwrap()][4..];
    if &hcl[0..1] != "#" {
        return false;
    }
    if hex::decode(&hcl[1..]).is_err() {
        return false;
    }

    //ecl
    let correct_cols = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    let ecl = &input[keys.iter().position(|x| *x == "ecl").unwrap()][4..];
    if !correct_cols.iter().any(|x| *x == ecl) {
        return false;
    }
    

    //pid
    let pid = &input[keys.iter().position(|x| *x == "pid").unwrap()][4..];
    println!("{}", pid);
    if pid.parse::<usize>().is_err() {
        return false;
    }
    if pid.len() != 9 {
        return false;
    }
    println!("everything correct");


    return true;
}

fn main() {
    let filename = "data/input.txt";
    let contents = fs::read_to_string(filename).expect("Could not read the file");
    let contents = contents.replace(" ", "\r\n");
    let content_split = contents.trim().split("\r\n\r\n").collect::<Vec<_>>();
    let data = content_split.iter().map(|x| x.split("\r\n").collect::<Vec<_>>()).collect::<Vec<_>>();

    let nr_enough : usize = data.iter().map(|x| check_7_field_presence(x) as usize).sum();
    let nr_correct : usize = data.iter().map(|x| check_correct_fields_present(x) as usize).sum();

    println!("Part 1: {}", nr_enough);
    println!("Part 2: {}", nr_correct);
}
