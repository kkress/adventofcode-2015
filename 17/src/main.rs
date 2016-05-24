use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn sum(sizes: &[u32]) -> u32 {
    sizes.iter().fold(0, |acc, v| acc + v)
}

#[test]
fn test_sum() {
    assert_eq!(150, sum(&vec![10u32, 20u32, 30u32, 40u32, 50u32]));
}



fn combination(count: u32, from: &[u32]) -> Vec<Vec<u32>> {
    let mut combos: Vec<Vec<u32>> = Vec::new();
    if count == 1 {
        for i in from {
            combos.push(vec![*i]);
        }
    } else if count > 1 {
        for i in 0..from.len() {
            let sub_combos = combination(count - 1, &from[i + 1..]);
            for c in &sub_combos {
                let mut head = Vec::new();
                head.push(from[i]);
                head.extend(c);
                combos.push(head);
            }
        }
    }
    combos
}


fn part1_combos(eggnog: &u32, sizes: &Vec<u32>) {
    let mut count = 0;
    for i in 0..sizes.len() {
        for c in combination(i as u32, &sizes) {
            if sum(&c) == *eggnog {
                println!("150 == {:?}", c);
                count += 1;
            }
        }
    }
    println!("There are {} combos", count);
}

fn main() {
    let f = File::open("input.txt").unwrap();
    let line_buffer = BufReader::new(&f);

    let mut containers = Vec::new();
    let eggnog = 150u32;

    for line in line_buffer.lines() {
        let size = line.unwrap().parse::<u32>().unwrap();
        containers.push(size);
    }

    part1_combos(&eggnog, &containers);
}
