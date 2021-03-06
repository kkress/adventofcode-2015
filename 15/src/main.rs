use std::cmp::max;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
struct Cookie {
    total_capacity: i32,
    ingredients: Vec<Ingredient>,
}

impl Cookie {
    fn new(capacity: i32, ingredients: Vec<Ingredient>) -> Cookie {
        Cookie {
            total_capacity: capacity,
            ingredients: ingredients,
        }
    }
    fn calories(&self, distribution: &Vec<i32>) -> i32 {
        (0..distribution.len()).fold(0,
                                     |acc, x| acc + self.ingredients[x].calories * distribution[x])
    }

    fn best_calories(&self, exact: i32) -> (Vec<i32>, i32) {
        let all = generate_possibilities(self.total_capacity, self.ingredients.len() as i32)
            .into_iter()
            .filter(|x| self.calories(x) == exact)
            .collect::<Vec<_>>();
        let best = all.iter().map(|x| (x, self.score(x))).max_by_key(|x| x.1).unwrap();
        return (best.0.clone(), best.1);
    }

    fn best(&self) -> (Vec<i32>, i32) {
        let all = generate_possibilities(self.total_capacity, self.ingredients.len() as i32);
        let best = all.iter().map(|x| (x, self.score(x))).max_by_key(|x| x.1).unwrap();
        println!("Best? {:?}", best);
        return (best.0.clone(), best.1);
    }

    fn score(&self, distribution: &Vec<i32>) -> i32 {
        let cap = max(0,
                      (0..distribution.len()).fold(0, |acc, x| {
                          acc + self.ingredients[x].capacity * distribution[x]
                      }));
        let dur = max(0,
                      (0..distribution.len()).fold(0, |acc, x| {
                          acc + self.ingredients[x].durability * distribution[x]
                      }));
        let flav = max(0,
                       (0..distribution.len()).fold(0, |acc, x| {
                           acc + self.ingredients[x].flavor * distribution[x]
                       }));
        let text = max(0,
                       (0..distribution.len()).fold(0, |acc, x| {
                           acc + self.ingredients[x].texture * distribution[x]
                       }));
        return cap * dur * flav * text;
    }
}

#[derive(Debug)]
struct Ingredient {
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl Ingredient {
    fn new(name: String,
           capacity: i32,
           durability: i32,
           flavor: i32,
           texture: i32,
           calories: i32)
           -> Ingredient {
        Ingredient {
            name: name,
            capacity: capacity,
            durability: durability,
            flavor: flavor,
            texture: texture,
            calories: calories,
        }
    }
}

fn generate_possibilities(amount: i32, count: i32) -> Vec<Vec<i32>> {
    let mut options = Vec::new();
    if count == 1 {
        options = vec![vec![amount]]
    } else {
        for curr in 0..amount + 1 {
            let poss = generate_possibilities(amount - curr, count - 1)
                .into_iter()
                .map(|mut x| {
                    x.insert(0, curr);
                    x
                })
                .collect::<Vec<_>>();
            options.extend(poss.iter().cloned());
        }
    }
    options
}

fn main() {
    let f = File::open("input.txt").unwrap();
    let line_buffer = BufReader::new(&f);

    let mut ingredients = Vec::new();

    for line in line_buffer.lines() {
        let curr = line.unwrap();
        // Chocolate: capacity 0, durability 0, flavor -2, texture 2, calories 8
        let name_parts = curr.splitn(2, ':').collect::<Vec<_>>();
        let name = name_parts[0].to_string();
        let parts = name_parts[1]
            .split(',')
            .map(|x| x.trim().split_whitespace().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let capacity = parts[0][1].parse::<i32>().unwrap();
        let durability = parts[1][1].parse::<i32>().unwrap();
        let flavor = parts[2][1].parse::<i32>().unwrap();
        let texture = parts[3][1].parse::<i32>().unwrap();
        let calories = parts[4][1].parse::<i32>().unwrap();
        ingredients.push(Ingredient::new(name, capacity, durability, flavor, texture, calories));
    }
    let cookie = Cookie::new(100, ingredients);

    println!("Best overall: {:?}", cookie.best());
    println!("Best 500 cal: {:?}", cookie.best_calories(500));
}
