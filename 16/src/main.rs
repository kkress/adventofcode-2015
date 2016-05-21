use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;

fn is_fuzzy_match(all: &HashMap<&str, i32>, partial: &HashMap<&str, i32>) -> bool {
   for (k, v) in partial {
      if !match all.get(k) {
         Some(val) => val == v,
         None => false
      } { return false }
   }
   return true
}

fn main() {
    let f = File::open("input.txt").unwrap();
    let line_buffer = BufReader::new(&f);
   
    let mut search = HashMap::new();
    search.insert("children", 3);
    search.insert("cats", 7);
    search.insert("samoyeds", 2);
    search.insert("pomeranians", 3);
    search.insert("akitas", 0);
    search.insert("vizslas", 0);
    search.insert("goldfish", 5);
    search.insert("trees", 3);
    search.insert("cars", 2);
    search.insert("perfumes", 1);

    for line in line_buffer.lines() {
        let curr = line.unwrap();
        // Sue 21: vizslas: 3, cars: 7, akitas: 3
        let name_parts = curr.splitn(2, ':').collect::<Vec<_>>();
        let number = name_parts[0].split(' ').collect::<Vec<_>>()[1].parse::<i32>().unwrap();
        let parts = name_parts[1]
            .split(',')
            .map(|x| {
               x.trim().split_whitespace().map(|x| x.trim_matches(':')).collect::<Vec<_>>()
            }).collect::<Vec<_>>();
        let mut attr = HashMap::new();
        for p in parts {
          attr.insert(p[0], p[1].parse::<i32>().unwrap());
        }

        if is_fuzzy_match(&search, &attr) {
          println!("Match Sue #{} {:?}", number, attr); 
        }
    }
}
