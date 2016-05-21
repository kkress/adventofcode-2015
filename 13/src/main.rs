use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn score(names: &Vec<String>, happy: &HashMap<(String, String), i32>) -> i32 {
    let mut total = 0;
    for i in 0..names.len() {
        let curr = &names[i].clone();
        let left = if i == 0 {
            &names[names.len() - 1]
        } else {
            &names[i - 1]
        };
        let right = if i == &names.len() - 1 {
            &names[0]
        } else {
            &names[i + 1]
        };
        let score = happy.get(&(curr.clone(), left.clone())).unwrap() +
                    happy.get(&(curr.clone(), right.clone())).unwrap();
        total += score;
    }
    total
}

fn permutations(names: &Vec<String>) -> Vec<Vec<String>> {
    let mut clone = names.clone();
    let mut all = Vec::new();
    for i in 0..names.len() {
        let curr = clone.remove(i);
        let perms = permutations(&clone);
        if perms.len() == 0 {
            all.push(vec![curr.clone()]);
        }
        for p in &perms {
            let mut new = vec![curr.clone()];
            new.extend(p.iter().cloned());
            all.push(new);
        }
        clone.insert(i, curr);
    }
    return all;
}

fn main() {
    let f = File::open("input.txt").unwrap();
    let line_buffer = BufReader::new(&f);

    let mut happiness = HashMap::new();
    let mut names = Vec::new();

    for line in line_buffer.lines() {
        let curr = line.unwrap();
        let parts: Vec<&str> = curr.split(" ").collect();
        let who = parts[0].to_string();
        let dir = match parts[2] {
            "gain" => 1i32,
            "lose" => -1i32,
            _ => panic!("Unknown happiness modifier"),
        };
        let value = parts[3].parse::<i32>().unwrap() * dir;
        let other = parts[10][0..parts[10].len() - 1].to_string();

        if !names.contains(&who) {
            names.push(who.clone());
        }
        happiness.insert((who.clone(), other.clone()), value);
    }

    let part_two = true;
    if part_two {
        for n in &names {
            happiness.insert(("me".to_string(), n.clone()), 0);
            happiness.insert((n.clone(), "me".to_string()), 0);
        }
        names.push("me".to_string());
    }

    let all = permutations(&names);

    let mut max = 0;
    let mut max_list = Vec::new();
    for a in all {
        let sc = score(&a, &happiness);
        if sc > max {
            max = sc;
            max_list = a.clone();
        }
    }
    println!("{} for {:?}", max, &max_list);
}
