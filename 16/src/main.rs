use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;

fn is_fuzzy_match(target: &HashMap<&str, i32>, candidate: &HashMap<&str, i32>) -> bool {
    for (k, v) in candidate {
        if !match target.get(k) {
            Some(val) => val == v,
            None => false,
        } {
            return false;
        }
    }
    return true;
}

fn is_very_fuzzy_match(target: &HashMap<&str, i32>, candidate: &HashMap<&str, i32>) -> bool {
    for (k, v) in candidate {
        let target_val = target.get(k).unwrap();
        let is_match = match *k {
            "cats" | "trees" => v > target_val,
            "pomeranians" | "goldfish" => v < target_val,
            _ => v == target_val,
        };
        if !is_match {
            return false;
        }
    }
    return true;
}

fn main() {
    let f = File::open("input.txt").unwrap();
    let line_buffer = BufReader::new(&f);

    let mut target = HashMap::new();
    target.insert("children", 3);
    target.insert("cats", 7);
    target.insert("samoyeds", 2);
    target.insert("pomeranians", 3);
    target.insert("akitas", 0);
    target.insert("vizslas", 0);
    target.insert("goldfish", 5);
    target.insert("trees", 3);
    target.insert("cars", 2);
    target.insert("perfumes", 1);

    for line in line_buffer.lines() {
        let curr = line.unwrap();
        // Sue 21: vizslas: 3, cars: 7, akitas: 3
        let name_parts = curr.splitn(2, ':').collect::<Vec<_>>();
        let number = name_parts[0].split(' ').collect::<Vec<_>>()[1].parse::<i32>().unwrap();
        let parts = name_parts[1]
            .split(',')
            .map(|x| x.trim().split_whitespace().map(|x| x.trim_matches(':')).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut attr = HashMap::new();
        for p in parts {
            attr.insert(p[0], p[1].parse::<i32>().unwrap());
        }

        if is_fuzzy_match(&target, &attr) {
            println!("Part1 Match Sue #{} {:?}", number, attr);
        }

        if is_very_fuzzy_match(&target, &attr) {
            println!("Part2 Match Sue #{} {:?}", number, attr);
        }
    }
}
