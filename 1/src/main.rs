use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut f = File::open("input.txt").unwrap();

    let mut input = String::new();
    let size = f.read_to_string(&mut input).unwrap();
    let mut floor = 0;
    let mut counter = 0;
    let mut entered_basement = false;
    for c in input.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!("Invalid input char {}"),
        }
        counter += 1;
        if !entered_basement && floor < 0 {
            println!("Entered basement on position {}", counter);
            entered_basement = true;
        }
    }
    println!("With {} instructions we ended on floor {}", size, floor);
}
