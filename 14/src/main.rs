use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
struct Fly {
    name: String,
    speed: i32,
    time: i32,
    rest: i32,
}

impl Fly {
    fn new(name: String, speed: i32, time: i32, rest: i32) -> Fly {
        Fly {
            name: name,
            speed: speed,
            time: time,
            rest: rest,
        }
    }

    fn distance_at_time(&self, seconds: i32) -> i32 {
        let full_cycle = self.time + self.rest;
        let full_cycle_distance = self.time * self.speed;
        let count = seconds / full_cycle;
        let rem = seconds % full_cycle;
        if rem < self.time {
            full_cycle_distance * count + (self.speed * rem)
        } else {
            full_cycle_distance * (count + 1)
        }
    }
}

fn main() {
    let f = File::open("input.txt").unwrap();
    let line_buffer = BufReader::new(&f);

    let mut racers = Vec::new();

    for line in line_buffer.lines() {
        let curr = line.unwrap();
        let parts: Vec<&str> = curr.split(" ").collect();
        // Dancer can fly 27 km/s for 5 seconds, but then must rest for 132 seconds.
        let who = parts[0].to_string();
        let speed = parts[3].parse::<i32>().unwrap();
        let time = parts[6].parse::<i32>().unwrap();
        let rest = parts[13].parse::<i32>().unwrap();

        let fly = Fly::new(who, speed, time, rest);
        racers.push(fly);
    }

    let end = 2503;

    let part1 = racers.iter().map(|x| x.distance_at_time(end)).max().unwrap();
    println!("Part 1: {}", part1);

    let mut scores = vec![0; racers.len()];
    for i in 1..end + 1 {
        let curr_scores: Vec<(usize, i32)> =
            racers.iter().enumerate().map(|x| (x.0, x.1.distance_at_time(i))).collect();
        let max = curr_scores.iter().max_by_key(|&x| x.1).unwrap().1;
        let maxes = curr_scores.iter().filter(|&x| x.1 == max).collect::<Vec<_>>();
        for m in maxes {
            scores[m.0] += 1;
        }
    }

    println!("Part 2: {:?}", scores.iter().max().unwrap());
}
