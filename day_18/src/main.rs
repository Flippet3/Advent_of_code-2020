use std::fs;
use std::time::Instant;

fn add_brackets(eq : Vec<isize>, mut it: usize) -> Vec<isize> {
    loop {
        if it == eq.len() {
            return eq;
        }
        if eq[it] == -1 { // A plus is found
            // Gotto find the left position to insert a ( and the right position to insert a )
            let mut new_vect = vec![];
            let mut depth = 0;
            let mut new_it = it -1;
            // Find position of (
            loop {
                if eq[new_it] == -3 {
                    depth += 1;
                } else if eq[new_it] == -4 {
                    depth -= 1;
                }
                if depth < 1 {
                    break;
                }
                new_it -= 1;
            }
            new_vect.append(&mut eq[..new_it].to_vec());
            new_vect.push(-4);
            new_vect.append(&mut eq[new_it..it].to_vec());

            // Find position of )
            let mut depth = 0;
            let mut new_it = it + 1;
            loop {
                if eq[new_it] == -4 {
                    depth += 1;
                } else if eq[new_it] == -3 {
                    depth -= 1;
                }
                if depth < 1 {
                    new_it += 1;
                    break;
                }
                new_it += 1;
            }
            new_vect.append(&mut eq[it..new_it].to_vec());
            new_vect.push(-3);
            new_vect.append(&mut eq[new_it..].to_vec());
            return add_brackets(new_vect, it + 2);
        }
        it += 1;
    }
}

fn solve(eq : Vec<isize>) -> isize {
    if eq.len() == 1 {
        return eq[0];
    }
    if eq.contains(&-3) {
        let mut it = eq.iter().position(|x| *x == -3).unwrap();
        let it_0 = it.clone();
        loop {
            it -= 1;
            if eq[it] == -4 {
                let mut new_vec = eq[0..it as usize].to_vec();
                new_vec.push(solve(eq[it+1..it_0].to_vec()));
                new_vec.append(&mut eq[it_0+1..].to_vec());
                return solve(new_vec);
            }
        }
    } else {
        let mut new_vec = vec![];
        if eq[1] == -2 {
            new_vec.push(eq[0] * eq[2]);
        } else {
            new_vec.push(eq[0] + eq[2]);
        }
        if eq.len() == 3 {
            return new_vec[0];
        }
        new_vec.append(&mut eq[3..].to_vec());
        return solve(new_vec);
    }
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
    // ( = -4;    // ) = -3;    // * = -2;    // + = -1;
    let data = contents.trim().split("\r\n").map(|x| x.replace(" ", "").chars().collect::<Vec<_>>().iter().map(|x| if *x == '(' {-4} else if *x == ')' {-3} else if *x == '*' {-2} else if *x == '+' {-1} else {*x as isize-48}).collect::<Vec<_>>()).collect::<Vec<_>>();

    let now = Instant::now();
    let answers = data.to_vec().iter().map(|x| solve(x.to_vec())).collect::<Vec<_>>();
    let answers2 = data.to_vec().iter().map(|x| solve(add_brackets(x.to_vec(),0))).collect::<Vec<_>>();
    let answer1 : isize = answers.iter().sum();
    let answer2 : isize = answers2.iter().sum();

    println!("{}", now.elapsed().as_secs_f64());

    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);
}
