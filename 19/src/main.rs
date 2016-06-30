use regex::Regex;
use std::cmp::min;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashSet;
use std::str::FromStr;

extern crate regex;

#[derive(Clone, Debug)]
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

    // Assuming this replacement was used to create result, what was it before.
    fn produced_from(&self, result: &String) -> Option<(usize, String)> {
        let new = result.replace(&self.val, &self.key);
        if new == *result {
            None
        } else {
            Some((result.matches(&self.val).count(), new))
        }
    }

    fn replace_first(&self, input: &String) -> Option<String> {
        let len = self.key.len();
        input.find(&self.key)
            .and_then(|x| Some(input[0..x].to_string() + &self.val + &input[x + len..]))
    }

    fn num_result_molecules(&self) -> usize {
        self.val.chars().filter(|x| x.is_uppercase()).count()
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

fn find_synthesis(input: &String, replacements: &Vec<Replacement>) -> Vec<Replacement> {
    let mut sorted = replacements.clone();
    sorted.sort_by(|a, b| a.num_result_molecules().cmp(&b.num_result_molecules()));
    return find_synthesis_with_order(input, &sorted);
}

fn find_synthesis_with_order(input: &String, replacements: &Vec<Replacement>) -> Vec<Replacement> {
    let mut curr = input.clone();
    let mut steps = Vec::new();
    loop {
        let (next, mut next_steps) = best_step(&curr, replacements);
        println!("Best Steps {:?} from {} to {}", next_steps, curr, next);
        next_steps.extend(steps);
        steps = next_steps;
        if next == "e" {
            break;
        }
        curr = next;
    }
    steps
}

fn best_step(input: &String, sorted_replacements: &Vec<Replacement>) -> (String, Vec<Replacement>) {
    let mut curr = input;
    for repl in sorted_replacements {
        let replaced = repl.produced_from(curr);
        match replaced {
            None => {}
            Some((num, new)) => {
                let mut steps = Vec::new();
                for i in 0..num {
                    steps.push(repl.clone());
                }
                return (new, steps);
            }
        };
    }
    (input.clone(), Vec::new())
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

// Totally infeasible.
fn exhaustive(input: &String,
              target: &String,
              repls: &Vec<Replacement>,
              depth: usize)
              -> (HashSet<String>, Option<usize>) {
    let i_len = input.len();
    let t_len = target.len();
    if i_len > t_len {
        return (HashSet::new(), None);
    } else if i_len == t_len && input == target {
        return (HashSet::new(), Some(depth));
    }

    let mut out = HashSet::new();
    let mut best = None;
    for poss in unique_possibilities(input, &repls) {
        let (options, found) = exhaustive(&poss, target, &repls, depth + 1);
        best = match found {
            Some(v) => Some(min(best.unwrap_or(std::usize::MAX), v)),
            None => best,
        };
        out = out.union(&options).cloned().collect();
    }
    (out, best)
}

// Also infeasible
fn exhaustive_reverse(input: &String,
                      target: &String,
                      repls: &Vec<Replacement>,
                      depth: usize)
                      -> (HashSet<String>, Option<usize>) {
    let i_len = input.len();
    let t_len = target.len();
    if i_len < t_len {
        return (HashSet::new(), None);
    } else if i_len == t_len && input == target {
        println!("Bottom out {}", depth);
        return (HashSet::new(), Some(depth));
    }

    let mut out = HashSet::new();
    let mut best = None;
    for poss in unique_possibilities(input, &repls) {
        let (options, found) = exhaustive_reverse(&poss, target, &repls, depth + 1);
        best = match found {
            Some(v) => Some(min(best.unwrap_or(std::usize::MAX), v)),
            None => best,
        };
        out = out.union(&options).cloned().collect();
    }
    if best.is_some() {
        println!("Found one at depth {}", best.unwrap());
    }
    if out.len() == 0 {
        println!("Busted at {} with {}", depth, input);
    }
    (out, best)
}

// Cheat and sue the formula
// https://www.reddit.com/r/adventofcode/comments/3xflz8/day_19_solutions/cy4etju to reduce based
fn calculate(input: &String) -> usize {
    let element_count = input.chars().filter(|x| x.is_uppercase()).count();
    let paren_count = input.match_indices("Ar").count() + input.match_indices("Rn").count();
    let comma_count = input.match_indices("Y").count();
    element_count - paren_count - 2 * comma_count - 1
}

fn main() {
    let f = File::open("input.txt").unwrap();
    let line_buffer = BufReader::new(&f);

    let mut repls = Vec::new();
    let mut goal: String = "".to_string();
    for line in line_buffer.lines() {
        let curr = line.unwrap();
        if curr == "" {
            continue;
        }
        match curr.parse::<Replacement>() {
            Ok(r) => repls.push(r),
            Err(_) => {
                goal = curr;
            }
        };
    }

    println!("Repls {:?}", repls);

    let mut rev_reps = Vec::new();
    for r in &repls {
        rev_reps.push(Replacement {
            key: r.val.clone(),
            val: r.key.clone(),
        });
    }

    let possible = unique_possibilities(&goal, &repls);
    //    let ex = exhaustive_reverse(&goal, &"e".to_string(), &rev_reps, 0);
    // let chain = find_synthesis(&goal, &repls);
    println!("{} cheater, I'm not writing a lexer and I got the hard input",
             calculate(&goal));
}
