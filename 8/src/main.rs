use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn unescape_len(s: &str) -> usize {
    let mut itr = s.chars();
    let (start, end) = (itr.next(), itr.next_back());
    if start.unwrap() != '"' || end.unwrap() != '"' {
        panic!("Invalid string");
    }
    let mut count = 0;
    while let Some(next) = itr.next() {
        count += match next {
            '\\' => {
                match itr.next() {
                    Some('\\') => 1,
                    Some('"') => 1,
                    Some('x') => {
                        itr.next();
                        itr.next();
                        1
                    }
                    Some(x) => panic!("Unknown escape sequence: {}", x),
                    None => panic!("NOPE"),  
                }
            }
            x => 1,
        };
    }
    return count;
}

fn escape_len(s: &str) -> usize {
    let mut itr = s.chars();
    // min size with quotes
    let mut count = 2;
    while let Some(next) = itr.next() {
        count += match next {
            '\\' => 2,
            '"' => 2,
            _ => 1,
        };
    }
    return count;
}

fn main() {
    let f = File::open("input.txt").unwrap();
    let line_buffer = BufReader::new(&f);

    let mut total_orig = 0;
    let mut total_unesc = 0;
    let mut total_esc = 0;

    for line in line_buffer.lines() {
        let curr = line.unwrap();
        let orig_len = curr.len();
        let unesc_len = unescape_len(&curr);
        let esc_len = escape_len(&curr);
        println!("orig={} unesc={} esc={}", orig_len, unesc_len, esc_len);
        total_orig += orig_len;
        total_esc += esc_len;
        total_unesc += unesc_len;
    }
    println!("Total orig {}", total_orig);
    println!("Total esc {} -{}", total_unesc, (total_orig - total_unesc));
    println!("Total unesc {} +{}", total_esc, (total_esc - total_orig));
}

#[test]
fn known() {
    assert_eq!(unescape_len(r#""""#), 0);
    assert_eq!(unescape_len(r#""abc""#), 3);
    assert_eq!(unescape_len(r#""aaa\"aaa""#), 7);
    assert_eq!(unescape_len(r#""\x27""#), 1);

    assert_eq!(escape_len(r#""""#), 6);
    assert_eq!(escape_len(r#""abc""#), 9);
    assert_eq!(escape_len(r#""aaa\"aaa""#), 16);
    assert_eq!(escape_len(r#""\x27""#), 11);
}
