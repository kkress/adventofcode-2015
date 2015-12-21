
fn increment(c: char) -> char {
   let max = 'z' as u8;
   let inc = c as u8 + 1;
   return if inc > max {
      'a'
   } else {
      inc as char
   }
}

fn next_password(old: &String) -> String {
   let mut new = String::new();

   let mut inc_next = true;
   for c in old.chars().rev() {
      let mut next = c;
      if inc_next {
         next = increment(c);
         if next != 'a' {
            inc_next = false;
         }
      }
      new.push(next);
   }
   if inc_next {
      new.push('a');
   }
   return new.chars().rev().collect();
}

fn is_straight(s: &str) -> bool {
   if s.len() != 3 {
      return false;
   }
   let b = s.as_bytes();
   return b[0] + 1 == b[1] && b[1] + 1 == b[2];
}

fn has_straight(pass: &String) -> bool {
   for (i, _) in pass.chars().enumerate().skip(2) {
      if is_straight(&pass[i-2..i+1]) {
         return true;
      }
   }
   return false;
}

fn has_banned_letter(pass: &String) -> bool {
   return pass.chars().any(|x| {
      match x {
         'i' | 'o' | 'l' => true,
         _ => false,
      }
   });
}

fn is_pair(s: &str) -> bool {
   let b = s.as_bytes();
   return b[0] == b[1];
}

fn has_two_pair(pass: &String) -> bool {
   let mut prev = "--";
   let mut skip = false;
   let mut count = 0;
   for (i, _) in pass.chars().enumerate().skip(1) {
      if skip {
         skip = false;
         continue;
      }
      if is_pair(&pass[i-1..i+1]) {
         if prev != &pass[i-1..i+1] {
            count += 1;
            prev = &pass[i-1..i+1];
            skip = true;
         }
      }
   }
   return count > 1;
}

fn valid_password(pass: &String) -> bool {
   return has_straight(pass) && !has_banned_letter(pass) && has_two_pair(pass);
}

fn main() {
   let input = "cqjxjnds";
   let mut next = input.to_string();
   let mut count = 0;
   loop {
      next = next_password(&next);
      if valid_password(&next) {
         count += 1;
         println!("Next valid password is {}", next);
      }
      if count > 1 {
         break;
      }
   }
}

#[test]
fn test_increment() {
   assert_eq!('z', increment('y'));
   assert_eq!('a', increment('z'));
}

#[test]
fn test_new_pass() {
   assert_eq!("aa", next_password(&"z".to_string()));
   assert_eq!("abcdefh", next_password(&"abcdefg".to_string()));
   assert_eq!("aaaaaaa", next_password(&"zzzzzz".to_string()));
}

#[test]
fn test_is_staight() {
   assert_eq!(true, is_straight(&"abc".to_string()));
   assert_eq!(true, is_straight(&"lmn".to_string()));
   assert_eq!(false, is_straight(&"kfi".to_string()));
}

#[test]
fn test_has_staight() {
   assert_eq!(false, has_straight(&"z".to_string()));
   assert_eq!(false, has_straight(&"aaa".to_string()));
   assert_eq!(false, has_straight(&"abd".to_string()));
   assert_eq!(true, has_straight(&"aaabcd".to_string()));
}

#[test]
fn test_has_banned() {
   assert_eq!(false, has_banned_letter(&"z".to_string()));
   assert_eq!(false, has_banned_letter(&"aaa".to_string()));
   assert_eq!(false, has_banned_letter(&"abd".to_string()));
   assert_eq!(true, has_banned_letter(&"aaaicd".to_string()));
   assert_eq!(true, has_banned_letter(&"aaalcd".to_string()));
   assert_eq!(true, has_banned_letter(&"aaaocd".to_string()));
}

#[test]
fn test_is_pair() {
   assert_eq!(false, is_pair("ab"));
   assert_eq!(true, is_pair("aa"));
}

#[test]
fn test_has_two_pair() {
   assert_eq!(false, has_two_pair(&"abcdefg".to_string()));
   assert_eq!(false, has_two_pair(&"aaaa".to_string()));
   assert_eq!(false, has_two_pair(&"aaxaa".to_string()));
   assert_eq!(true, has_two_pair(&"aabbaa".to_string()));
}
