use regex::Regex;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashSet;
use std::str::FromStr;

extern crate regex;

struct Replacement {
    key: String,
    val: String,
}

impl Replacement {
    fn new(key: &str, val: &str) -> Replacement {
        Replacement {
            key: key.to_string(),
            val: val.to_string(),
        }
    }

    fn possibilities(&self, input: &String) -> HashSet<String> {
        let mut poss = HashSet::new();
        for p in input.match_indices(&self.key) {
            let len = p.1.len();
            let prefix = &input[0..p.0];
            let suffix = &input[p.0 + len..];
            poss.insert(prefix.to_string() + &self.val + suffix);
        }
        poss
    }
}

#[derive(Debug)]
struct InvalidInput;

impl FromStr for Replacement {
    type Err = InvalidInput;
    fn from_str(s: &str) -> Result<Replacement, InvalidInput> {
        let parser = Regex::new(r"(.+)\s=>\s(.+)").unwrap();
        let res = parser.captures(s);
        if res.is_none() {
            return Err(InvalidInput {});
        }

        let cap = res.unwrap();
        let from = cap.at(1).unwrap();
        let to = cap.at(2).unwrap();
        Ok(Replacement::new(from, to))
    }
}

fn unique_possibilities(input: &String, replacements: &Vec<Replacement>) -> HashSet<String> {
    let mut out = HashSet::new();
    for r in replacements {
        out = out.union(&r.possibilities(input)).cloned().collect();
    }
    out
}

#[test]
fn test_unique() {
    let start = "HOH".to_string();
    let reps = vec!(
        Replacement::new("H", "HO"),
        Replacement::new("H", "OH"),
        Replacement::new("O", "HH"),
    );
    let poss = unique_possibilities(&start, &reps);
    assert_eq!(poss,
               ["HOOH", "OHOH", "HOHO", "HHHH"].iter().map(|x| x.to_string()).collect());
}


fn main() {
    let f = File::open("input.txt").unwrap();
    let line_buffer = BufReader::new(&f);

    let mut repls = Vec::new();
    let mut start: String = "".to_string();
    for line in line_buffer.lines() {
        let curr = line.unwrap();
        if curr == "" {
            continue;
        }
        match curr.parse::<Replacement>() {
            Ok(r) => repls.push(r),
            Err(_) => {
                start = curr;
            }
        };
    }

    let possible = unique_possibilities(&start, &repls);
    println!("{} possibilities", possible.len());
}
