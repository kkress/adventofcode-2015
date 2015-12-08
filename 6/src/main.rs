use regex::Regex;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::str::FromStr;
use std::fmt;

extern crate regex;

#[derive(Debug)]
enum Operation {
   On,
   Off,
   Toggle,
}

#[derive(Debug)]
struct Coordinate {
   x: usize,
   y: usize,
}

#[derive(Debug)]
struct ParseCoordinateError;

impl FromStr for Coordinate {
   type Err = ParseCoordinateError;
   fn from_str(s: &str) -> Result<Coordinate, ParseCoordinateError> {
      let parts: Vec<_> = s.split(",").collect();
      if parts.len() != 2 {
         Err(ParseCoordinateError)
      } else {
         Ok(Coordinate{
            x: parts[0].parse::<usize>().unwrap(),
            y: parts[1].parse::<usize>().unwrap(),
         })
                     
      }
   }
}

impl fmt::Display for Coordinate {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{},{}", self.x, self.y)
   }
}

// Works for both boolean and brigtness.
trait Light {
   fn on(&mut self);
   fn off(&mut self);
   fn toggle(&mut self);
   fn value(&self) -> u32;
}

// On/Off style lights.  value is 1 if on
impl Light for bool {
   fn toggle(&mut self) {
      *self = !*self;
   }
   fn on(&mut self) {
      *self = true;
   }
   fn off(&mut self) {
      *self = false;
   }
   fn value(&self) -> u32 {
      return if *self { 1 } else { 0 };
   }
}

// Brightness style lights, value is brigtness.
impl Light for u32 {
   fn toggle(&mut self) {
      *self += 2;
   }
   fn on(&mut self) {
      *self += 1;
   }
   fn off(&mut self) {
      if *self > 0 {
         *self -= 1;
      }
   }
   fn value(&self) -> u32 {
      return *self;
   }
}

fn apply<T: Light>(matrix: &mut [[T; 1000]; 1000], cmd: &Operation, start: &Coordinate, end: &Coordinate) {
   for i in start.x..end.x+1 {
      for j in start.y..end.y+1 {
         match *cmd {
            Operation::On => matrix[i][j].on(),
            Operation::Off => matrix[i][j].off(),
            Operation::Toggle => matrix[i][j].toggle(),
         };
      }
   }
}

fn total_value<T: Light>(matrix: &[[T; 1000]; 1000]) -> u32 {
   return matrix.iter()
      .fold(0, |sum, i| sum + i.iter()
         .fold(0, |inner_sum, j| inner_sum + j.value()));
}

fn main() {
   let f = File::open("input.txt").unwrap();
   let line_buffer = BufReader::new(&f);
   
   let mut part1: [[bool; 1000]; 1000] = [[false; 1000]; 1000];
   let mut part2: [[u32; 1000]; 1000] = [[0; 1000]; 1000];
   let parser = Regex::new(r"(turn on|toggle|turn off) (\d+,\d+) through (\d+,\d+)").unwrap();

   for line in line_buffer.lines() {
      let curr = line.unwrap();
      let cap = parser.captures(&curr).unwrap();
      let start = cap.at(2).unwrap().parse::<Coordinate>().ok().unwrap();
      let end = cap.at(3).unwrap().parse::<Coordinate>().ok().unwrap();

      let cmd = match cap.at(1).unwrap() {
         "turn on" => Operation::On,
         "turn off" => Operation::Off,
         "toggle" => Operation::Toggle,
         _ => panic!("Unknown command"),
      };
      apply(&mut part1, &cmd, &start, &end);
      apply(&mut part2, &cmd, &start, &end);
   }

   println!("part1: {}", total_value(&part1));
   println!("part2: {}", total_value(&part2));
}

#[test]
fn try_tests() {
   let mut matrix: [[bool; 1000]; 1000] = [[false; 1000]; 1000];
   apply(&mut matrix, &Operation::On, &Coordinate{x: 0, y: 0}, &Coordinate{x: 999, y:999});
   assert_eq!(1000000, total_value(&matrix));
   apply(&mut matrix, &Operation::Toggle, &Coordinate{x: 1, y: 1}, &Coordinate{x: 10, y:10});
   assert_eq!(999900, total_value(&matrix));
}

#[test]
fn try_part2() {
   let mut matrix: [[u32; 1000]; 1000] = [[0; 1000]; 1000];
   apply(&mut matrix, &Operation::On, &Coordinate{x: 0, y: 0}, &Coordinate{x: 999, y:999});
   assert_eq!(1000000, total_value(&matrix));
   apply(&mut matrix, &Operation::Toggle, &Coordinate{x: 1, y: 1}, &Coordinate{x: 10, y:10});
   assert_eq!(999900, total_value(&matrix));
}
