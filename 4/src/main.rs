extern crate crypto;

use crypto::digest::Digest;
use crypto::md5::Md5;

fn calculate_md5(digest: &mut Digest, input: &str, num: u64) -> String {
   digest.reset();
   digest.input(&format!("{}{}", input, num).into_bytes()[..]);
   return digest.result_str();
}

fn main() {
   let input = "yzbqklnj";
   let mut digest = Md5::new();
   let mut i = 0u64;
   loop {
      let result = calculate_md5(&mut digest, input, i);
      if result.starts_with("000000") {
         println!("Found a coin at {} = {}", i, result);
         break;
      }
      if i % 100000 == 0 {
         println!("{}", i);
      }
      i += 1;
   }
}
