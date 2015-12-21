extern crate rustc_serialize;

use rustc_serialize::json::Json;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn sum_values(data: &Json) -> i64 {
   return match data {
      &Json::I64(x) => x,
      &Json::U64(x) => x as i64,
      &Json::F64(x) => x as i64,
      &Json::String(_) => 0,
      &Json::Boolean(_) => 0,
      &Json::Array(ref a) => {
         a.iter().fold(0, |acc, ref x| acc + sum_values(&x))
      },
      &Json::Object(ref o) => {
         o.values().fold(0, |acc, ref x| acc + sum_values(&x))
      }
      &Json::Null => 0,
   }
}

fn sum_values_unred(data: &Json) -> i64 {
   return match data {
      &Json::I64(x) => x,
      &Json::U64(x) => x as i64,
      &Json::F64(x) => x as i64,
      &Json::String(_) => 0,
      &Json::Boolean(_) => 0,
      &Json::Array(ref a) => {
         a.iter().fold(0, |acc, ref x| acc + sum_values_unred(&x))
      },
      &Json::Object(ref o) => {
         if o.values().any(|ref x| match *x {
               &Json::String(ref s) => s == "red",
               _ => false,
            }) {
            0
         } else {
            o.values().fold(0, |acc, ref x| acc + sum_values_unred(&x))
         }
      }
      &Json::Null => 0,
   }
}

fn main() {
   let mut file = File::open("input.json").unwrap();
   let mut data = String::new();
   file.read_to_string(&mut data).unwrap();

   let json = Json::from_str(&data).unwrap();
   let val = sum_values_unred(&json);
   println!("Total value is {}", val);
}

#[test]
fn simple_nested() {
   let data = Json::from_str(&r#"{"a": 1, "b": {"a":1}}"#).unwrap();
   assert_eq!(sum_values(&data), 2);
}

#[test]
fn simple_array() {
   let data = Json::from_str(&r#"{"a": 1, "b": {"a":[1,2,3,4,5,6]}, "c":30}"#).unwrap();
   assert_eq!(sum_values(&data), 52);
}

#[test]
fn simple_array_unred() {
   let data = Json::from_str(r#"{"a": 1, "b": {"a":[1,2,3,4,5,6]}, "c":30}"#).unwrap();
   assert_eq!(sum_values_unred(&data), 52);
}

#[test]
fn nested_unred() {
   let data = Json::from_str(r#"{"a": 1, "b": {"a":[1,2,3,4,5,6], "a": "red", "b": 1}, "c":30}"#).unwrap();
   assert_eq!(sum_values_unred(&data), 31);
}
