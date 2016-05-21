use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn is_banned(prev: char, curr: char) -> bool {
    let banned = &["ab", "cd", "pq", "xy"];
    for b in banned {
        let mut itr = b.chars();
        if itr.next() == Some(prev) && itr.next() == Some(curr) {
            return true;
        }
    }
    return false;
}

fn part1_good(curr: &str) -> bool {
    let mut vowels = 0;
    let mut prev = '-';
    let mut double = false;
    for c in curr.chars() {
        if is_banned(prev, c) {
            return false;
        }
        vowels += match c {
            'a' | 'e' | 'i' | 'o' | 'u' => 1,
            _ => 0,
        };
        double = double || prev == c;
        prev = c;
    }
    return vowels >= 3 && double;
}

// This function assumes a huge amount about the vector:
// * That all pairs are added in scan order.
// * That all overlapping pairs are adjacent.
fn non_overlapping(v: &Vec<(usize, usize)>) -> bool {
    // Fold on non-overlapping + prev suffix.
    return v.len() > 2 ||
           v.iter().fold((false, v[0].0), |acc, &v| (acc.0 || (acc.1 != v.0), v.1)).0;
}

fn repeated_non_overlapping_pairs(m: HashMap<(char, char), Vec<(usize, usize)>>)
                                  -> HashMap<(char, char), Vec<(usize, usize)>> {
    return m.into_iter()
        .filter(|&(_, ref v)| v.len() > 1 && non_overlapping(v))
        .collect();
}

fn part2_good(curr: &str) -> bool {
    let chars: Vec<char> = curr.chars().collect();

    let mut repeated = false;
    let mut pairs: HashMap<(char, char), Vec<(usize, usize)>> = HashMap::new();
    for (i, _) in chars.iter().enumerate() {
        // Need to look at 0, 1 for 1.
        if i > 0 {
            let pair = (chars[i - 1], chars[i]);
            let location = (i - 1, i);
            let location_list = match pairs.entry(pair) {
                Vacant(entry) => entry.insert(vec![]),
                Occupied(entry) => entry.into_mut(),
            };
            location_list.push(location);
        }
        // Need to look at 0, 1, 2 for 2.
        if i > 1 {
            repeated = repeated || chars[i - 2] == chars[i];
        }
    }

    let r = repeated_non_overlapping_pairs(pairs);
    return repeated && r.len() > 0;
}


fn main() {
    let f = File::open("input.txt").unwrap();
    let line_buffer = BufReader::new(&f);

    let mut good_count_part1 = 0;
    let mut good_count_part2 = 0;
    for line in line_buffer.lines() {
        let curr = line.unwrap();
        if part1_good(&curr) {
            good_count_part1 += 1;
        }
        if part2_good(&curr) {
            good_count_part2 += 1;
            println!("{}", curr);
        }
    }
    println!("Total Good Part 1: {}", good_count_part1);
    println!("Total Good Part 2: {}", good_count_part2);
}

#[test]
fn check_good_part1() {
    assert_eq!(true, part1_good("ugknbfddgicrmopn"));
    assert_eq!(true, part1_good("aaa"));
    assert_eq!(false, part1_good("jchzalrnumimnmhp"));
    assert_eq!(false, part1_good("haegwjzuvuyypxyu"));
    assert_eq!(false, part1_good("dvszwmarrgswjxmb"));
}

#[test]
fn check_non_overlapping() {
    assert_eq!(false, non_overlapping(&vec![(0, 1), (1, 2)]));
    assert_eq!(true, non_overlapping(&vec![(0, 1), (2, 3)]));
    assert_eq!(true, non_overlapping(&vec![(0, 1), (6, 7)]));
    assert_eq!(true, non_overlapping(&vec![(0, 1), (1, 2), (2, 3)]));
}

#[test]
fn check_good_part2() {
    assert_eq!(false, part2_good("ugknbfddgicrmopn"));
    assert_eq!(false, part2_good("aaa"));
    assert_eq!(false, part2_good("jchzalrnumimnmhp"));
    assert_eq!(false, part2_good("haegwjzuvuyypxyu"));
    assert_eq!(false, part2_good("dvszwmarrgswjxmb"));

    assert_eq!(true, part2_good("qjhvhtzxzqqjkmpb"));
    assert_eq!(true, part2_good("xxyxx"));
    assert_eq!(false, part2_good("uurcxstgmygtbstg"));
    assert_eq!(false, part2_good("ieodomkazucvgmuy"));

    assert_eq!(true, part2_good("aaaaaaabaaaaaaa"));
    assert_eq!(true, part2_good("dconetwothreedceeabcb"));

    assert_eq!(true, part2_good("aaabcdedbc"));
    assert_eq!(true, part2_good("sknufchjdvccccta"));
}
