use std::cmp::{max, min};
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::str::FromStr;
use std::fmt;

const SIZE: usize = 100 as usize;

#[derive(Copy, Clone, Debug)]
struct Light {
    val: bool,
    stuck: bool,
}

impl Light {

    fn on(&mut self) {
        self.val = true;
    }

    fn off(&mut self) {
        self.val = false;
    }

    fn set(&mut self, state: bool) {
        self.val = state;
    }

    fn is_on(&self) -> bool {
        self.val || self.stuck
    }

    fn set_stuck(&mut self) {
        self.stuck = true;
    }

    fn next_state(&self, count: i32) -> bool {
        match count {
            2 if self.val => true,
            3 => true,
            _ => false,
        }
    }
}

impl fmt::Display for Light {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", if self.is_on() { '#' } else { '.' })
    }
}

struct LightArray {
    lights: [[Light; 100]; 100],
    stuck: bool,
}

impl LightArray {
    fn new() -> LightArray {
        LightArray{
            lights: [[Light{val:false, stuck: false}; SIZE]; SIZE],
            stuck: false,
        }
    }

    fn new_stuck() -> LightArray {
        let mut l = LightArray{
            lights: [[Light{val:false, stuck: false}; SIZE]; SIZE],
            stuck: true,
        };
        l.set_stuck();
        return l;
    }

    fn set_stuck(&mut self) {
        self.stuck = true;
        self.light_mut(0, 0).set_stuck();
        self.light_mut(0, SIZE-1).set_stuck();
        self.light_mut(SIZE-1, 0).set_stuck();
        self.light_mut(SIZE-1, SIZE-1).set_stuck();
    }

    fn light_mut(&mut self, x: usize, y: usize) -> &mut Light {
        &mut self.lights[x][y]
    }

    fn light(&self, x: usize, y: usize) -> &Light {
        &self.lights[x][y]
    }

    fn neighbors(&self, x: usize, y: usize) -> Vec<&Light> {
        let mut neighbors = Vec::new();
        let (start_x, end_x) = (if x == 0 { 0 } else { x -1 }, min(SIZE-1, x + 1));
        let (start_y, end_y) = (if y == 0 { 0 } else { y -1 }, min(SIZE-1, y + 1));
        for i in start_x..end_x+1 {
            for j in start_y..end_y+1 {
                if i != x || j != y {
                    neighbors.push(self.light(i, j));
                }
            }
        }
        neighbors
    }

    fn next(&self) -> LightArray {
        let mut next_array = if !self.stuck { LightArray::new()} else { LightArray::new_stuck() };
        for i in 0..SIZE {
            for j in 0..SIZE {
                let neighbors = self.neighbors(i, j);
                let on_count = neighbors.iter().fold(0, |sum, x| sum + if x.is_on() {1} else {0});
                next_array.light_mut(i,j).set(self.light(i, j).next_state(on_count));
            }
        }
        next_array
    }

    fn on_count(&self) -> i32 {
        let mut count = 0;
        for i in 0..SIZE {
            for j in 0..SIZE {
                if self.light(i, j).is_on() {
                    count += 1;
                }
            }
        }
        count
    }
}

impl fmt::Display for LightArray {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..SIZE {
            for j in 0..SIZE {
                write!(f, "{}", self.light(i, j));
            }
            write!(f, "\n");
        }
        Ok(())
    }
}

fn main() {
    let f = File::open("input.txt").unwrap();
    let line_buffer = BufReader::new(&f);

    let mut lights = LightArray::new();

    for (i, line) in line_buffer.lines().enumerate() {
        let curr = line.unwrap();
        for (j, chr) in curr.chars().enumerate() {
            lights.light_mut(i, j).set(match chr {
                '#' => true,
                '.' => false,
                _ => panic!("Unknown state!"),
            });
        }
    }

    run(&lights);

    lights.set_stuck();
    run(&lights);
}

fn run(orig: &LightArray) {
    let mut lights = orig.next();
    for i in 0..99 {
        lights = lights.next();
    }
    println!("100 iterations later {}", lights.on_count());
}
