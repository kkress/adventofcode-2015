use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug,Clone)]
struct Path {
   path: Vec<String>,
   distance: u32,
}

impl Path {
   fn push(&mut self, next: &String, dist: u32) {
      self.path.push(next.clone());
      self.distance += dist
   }

   fn best(one: Option<Path>, two: Path) -> Path {
      return match one {
         Some(x) => return if x.distance < two.distance { x } else { two },
         None => two,
      }
   } 
   fn worst(one: Option<Path>, two: Path) -> Path {
      return match one {
         Some(x) => return if x.distance > two.distance { x } else { two },
         None => two,
      }
   } 
}

#[derive(Debug)]
struct Map {
   distances: HashMap<(String, String), u32>,
   places: HashSet<String>,
}

impl Map {
   fn distance(&self, a: &String, b: &String) -> u32 {
      let d = *self.distances.get(&(a.clone(),b.clone())).unwrap();
      return d;
   }

   fn shortest_route(&self, left: &HashSet<String>, prev: Path) -> Path {
      if left.len() == 0 {
         return prev;
      }

      let mut best: Option<Path> = None;
      for next in left {
         let mut distance = 0;
         if prev.path.len() != 0 {
            distance = self.distance(&prev.path.last().unwrap(), next);
         }
         let mut next_path = prev.clone();
         next_path.push(next, distance);
         let mut next_left: HashSet<String> = left.clone();
         next_left.remove(next);
         best = Some(Path::best(best, self.shortest_route(&next_left, next_path)));
      }
      return best.unwrap();
   }

   fn longest_route(&self, left: &HashSet<String>, prev: Path) -> Path {
      if left.len() == 0 {
         return prev;
      }

      let mut worst: Option<Path> = None;
      for next in left {
         let mut distance = 0;
         if prev.path.len() != 0 {
            distance = self.distance(&prev.path.last().unwrap(), next);
         }
         let mut next_path = prev.clone();
         next_path.push(next, distance);
         let mut next_left: HashSet<String> = left.clone();
         next_left.remove(next);
         worst = Some(Path::worst(worst, self.longest_route(&next_left, next_path)));
      }
      return worst.unwrap();
   }
}

fn main() {
   let f = File::open("input.txt").unwrap();
   let line_buffer = BufReader::new(&f);

   let mut distances = HashMap::new();
   let mut places = HashSet::new();
 
   for line in line_buffer.lines() {
      let curr = line.unwrap();
      let parts: Vec<&str> = curr.split(" ").collect();
      let to = parts[0].to_string();
      let from = parts[2].to_string();
      let dist = parts[4].parse::<u32>().unwrap();
      distances.insert((to.clone(), from.clone()), dist);
      distances.insert((from.clone(), to.clone()), dist);
      places.insert(to);
      places.insert(from);
   }

   println!("distances {:?}", distances);
   println!("places {:?}", places);
   let map = Map{distances: distances, places: places};
   let short = map.shortest_route(&map.places, Path{path: vec![], distance: 0});
   let long = map.longest_route(&map.places, Path{path: vec![], distance: 0});
   println!("Shortest {:?}", short);
   println!("Longest {:?}", long);
}
