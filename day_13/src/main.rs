use std::fs;
use std::time::Instant;

fn find_closest_bus(arrival_time : usize, bus_ids : Vec<usize>) -> (usize, usize) {
    let mut it = arrival_time;
    let bus_id;
    let minutes_waited;
    loop {
        it += 1;
        for id in &bus_ids {
            if it%id == 0 {
                bus_id = id;
                minutes_waited = it-arrival_time;
                return (minutes_waited, *bus_id);
            }
        }
    }    
}

fn find_fitting_time_stamp (schedule : Vec<&str>) -> usize {
    let mut base = 0;
    let mut step = schedule[0].parse::<usize>().unwrap();
    let mut fitting_time_stamp = 0;
    for i in 1..schedule.len() {
        if schedule[i] == "x" {
            continue;
        }
        let this_id = schedule[i].parse::<usize>().unwrap();
        let mut it = 0;
        let mut base_changed = false;
        let mut base_i = 0;
        let mut new_base = 5;
        loop {
            if (base + step*it + i)%this_id == 0 {
                if i == (schedule.len() - 1) {
                    fitting_time_stamp = base + step*it;
                    break;
                }
                if !base_changed {
                    new_base = base + step * it;
                    base_i = base + step * it;
                    base_changed = true;
                } else {
                    step = base + step * it - base_i;
                    break;
                }
            }

            it += 1;
        }
        base = new_base;
    }
    return fitting_time_stamp;
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
    let data = contents.trim().split("\r\n").collect::<Vec<_>>();
    let arrival_time = data[0].parse::<usize>().unwrap();
    let mut bus_ids = data[1].split(",").collect::<Vec<_>>();
    let schedule = bus_ids.to_vec();
    bus_ids.retain(|&x| !x.parse::<usize>().is_err());
    let bus_ids = bus_ids.iter().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();

    let now = Instant::now();
    let (minutes_waited, bus_id) = find_closest_bus(arrival_time, bus_ids);
    let answer1 = minutes_waited * bus_id;
    let answer2 = find_fitting_time_stamp(schedule);
    println!("{}", now.elapsed().as_secs_f64());

    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);
}
