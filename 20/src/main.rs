fn all_factors(num: usize) -> Vec<usize> {
    let ceiling = (num as f64).sqrt() as usize;
    let mut factors = Vec::new();
    for curr in 1..num+1 {
        if num % curr == 0 {
            let div = num / curr;
            if div > curr {
                factors.push(curr);
                factors.push(div);
            } else if curr == div {
                factors.push(curr);
            } else {
                break;
            }
        }
        if curr > ceiling {
            break;
        }
    }
    factors.sort();
    factors
}

#[test]
fn test_factors() {
    assert_eq!(all_factors(1), vec!(1));
    assert_eq!(all_factors(2), vec!(1, 2));
    assert_eq!(all_factors(3), vec!(1, 3));
    assert_eq!(all_factors(4), vec!(1, 2, 4));
    assert_eq!(all_factors(5), vec!(1, 5));
    assert_eq!(all_factors(6), vec!(1, 2, 3, 6));
    assert_eq!(all_factors(7), vec!(1, 7));
    assert_eq!(all_factors(8), vec!(1, 2, 4, 8));
    assert_eq!(all_factors(9), vec!(1, 3, 9));
    assert_eq!(all_factors(10), vec!(1, 2, 5, 10));
}

fn count(num: usize) -> usize {
    all_factors(num).iter().fold(0, |acc, x| acc+x*10)
}

fn main() {
    let target = 33100000;
    let mut i = 1;
    let mut last = 0;
    loop {
        last = count(i);
        if i % 10000 == 0 {
            println!("{} = {}", i, last);
        }
        if last >= target {
            break;
        }
        i += 1;
    }
    println!("{} = {}", i, last);
}
