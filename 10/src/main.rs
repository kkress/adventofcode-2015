extern crate num;

use num::bigint::{BigUint, ToBigUint};

fn look_say(look: BigUint) -> BigUint {
    let mut prev: Option<char> = None;
    let mut count = 0;

    let mut out_str = String::new();
    for c in look.to_string().chars() {
        match prev {
            None => {
                count = 1;
            }
            Some(x) => {
                if x == c {
                    count += 1;
                } else {
                    out_str = out_str + &count.to_string() + &x.to_string();
                    count = 1;
                }
            }
        }
        prev = Some(c);
    }
    out_str = out_str + &count.to_string() + &prev.unwrap().to_string();
    return out_str.parse::<BigUint>().unwrap();
}

fn look_say_str(look: String) -> String {
    let mut prev: Option<char> = None;
    let mut count = 0;

    let mut out_str = String::new();
    for c in look.chars() {
        match prev {
            None => {
                count = 1;
            }
            Some(x) => {
                if x == c {
                    count += 1;
                } else {
                    out_str = out_str + &count.to_string() + &x.to_string();
                    count = 1;
                }
            }
        }
        prev = Some(c);
    }
    out_str = out_str + &count.to_string() + &prev.unwrap().to_string();
    return out_str;
}

fn main() {
    let input = "1113122113".to_string();

    let mut last = input;
    let mut count = 0;
    for i in 0..50 {
        let curr = look_say_str(last);
        last = curr;
        count += 1;
        // println!("[{}]:{} => {}", count, last.len(), last);
        println!("[{}]:{}", count, last.len());
    }
}

#[test]
fn look_test() {
    // assert_eq!(look_say(1.to_biguint()), 11.to_biguint());
    // assert_eq!(look_say(11.to_biguint()), 21.to_biguint());
    // assert_eq!(look_say(21.to_biguint()), 1211.to_biguint());
    // assert_eq!(look_say(1211.to_biguint()), 111221.to_biguint());
    // assert_eq!(look_say(111221.to_biguint()), 312211.to_biguint());
    //

    assert_eq!(look_say_str("1".to_string()), "11".to_string());
    assert_eq!(look_say_str("11".to_string()), "21".to_string());
    assert_eq!(look_say_str("21".to_string()), "1211".to_string());
    assert_eq!(look_say_str("1211".to_string()), "111221".to_string());
    assert_eq!(look_say_str("111221".to_string()), "312211".to_string());
}
