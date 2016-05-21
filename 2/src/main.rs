use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() {
    let f = File::open("input.txt").unwrap();
    let line_buffer = BufReader::new(&f);

    let mut per_present = Vec::new();
    for line in line_buffer.lines() {
        let curr = line.unwrap();
        let dimensions: Vec<u32> = curr.split("x").map(|s| s.parse::<u32>().unwrap()).collect();
        if dimensions.len() != 3 {
            panic!("Invalid input line {}", curr);
        }
        let (l, w, h) = (dimensions[0], dimensions[1], dimensions[2]);

        // Store each side.
        let sides = vec![(l, w), (w, h), (h, l)];

        // area of each side is x * y
        let mut areas: Vec<u32> = sides.iter().map(|x| x.0 * x.1).collect();
        areas.sort();

        // perimeter of each side is 2x + 2y
        let mut perimeters: Vec<u32> = sides.iter().map(|x| x.0 * 2 + x.1 * 2).collect();
        perimeters.sort();

        // smallest side by area.
        let slack = areas[0];

        // surfase area of the whole is 2* each side's area.
        let area = areas.iter().fold(0, |a, b| a + 2 * b);

        // volume is product of dimensions.
        let volume: u32 = dimensions.iter().fold(1, |a, b| a * b);

        // smallest side by perimeter;
        let min_perimeter = perimeters[0];

        // ribbon is min_perimeter face + volume;
        let ribbon = min_perimeter + volume;

        // paper required is area + slack
        let paper = area + slack;

        println!("{} area={} slack={} volume={}, ribbon={}, paper={}",
                 curr,
                 area,
                 slack,
                 volume,
                 ribbon,
                 paper);
        per_present.push((paper, ribbon));
    }
    let total = per_present.iter().fold((0, 0), |a, b| (a.0 + b.0, a.1 + b.1));
    println!("Total paper: {}", total.0);
    println!("Total Ribbon: {}", total.1);
}
