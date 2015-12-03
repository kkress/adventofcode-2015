use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

fn main() {
   let mut f = File::open("input.txt").unwrap();

   let mut input = String::new();
   let size = f.read_to_string(&mut input).unwrap();
   let mut map = HashMap::new();


   let mut santa = (0,0);
   let mut robo_santa = (0,0);
   map.insert(santa, 2);

   let mut count = 0;
   for c in input.chars() {
      let curr = if count % 2 == 0 { &mut santa } else { &mut robo_santa };
      let delta = match c {
         '^' => (0, 1),
         '>' => (1, 0),
         'v' => (0, -1),
         '<' => (-1, 0),
         _ => panic!("Invalid input char {}"),
      };
      
      *curr = (curr.0 + delta.0, curr.1 + delta.1);
      let value = match map.get(&*curr) {
         Some(&v) => v + 1,
         None => 1,
      };
      map.insert(*curr, value);
      count += 1;
   }
   println!("There are {} values", map.len());
}
