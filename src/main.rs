

use std::{ collections::{HashSet}};
mod numbers;
mod array;
mod strings;
mod map;



























// //mising array number
// fn missing_number(s : Vec<i32>)-> i32{
//    let num = s.len() as i32 + 1;
//    let arra = num * (num + 1) / 2 ;
//    let sums : i32 = s.iter().sum();
//    arra - sums
// }










// //Longest palindrome 
// // Find the longest palindromic substring
// fn longest_palindrome(s : &str)-> String{
  
   
//    let str : Vec<char> = s.chars().collect();
//    let mut best_start = 0 ;
//    let mut best_len = 0; 
//    fn expand_around(chars : &Vec<char> ,mut left : isize ,mut right : isize)->(usize , usize){
//       while left >= 0 
//          && right < chars.len() as isize
//          && chars[left as usize] == chars[right as usize]
//          {
//           left -= 1 ;
//           right += 1
//           }
//     ((left + 1) as usize , (right - 1 ) as usize)
//    }
//    for i in 0..str.len(){
//       let (l , r) = expand_around(&str,i as isize, i as isize);
//       if r - i + 1 > best_len{
//          best_start = l ;
//          best_len = r - l + 1;

//       }
//       let (l1 , r1) = expand_around(&str, i as isize, i as isize + 1);
//       if r1 >= l1 && r1 - l1 + 1 > best_len{
//          best_start = l1;
//          best_len = r1 - l1 + 1;
//       }
//    }

// str[best_start..best_start + best_len].iter().collect()


//    }



fn main(){
   
    numbers::numbers::numbers_operatins();
    array::array::array_operation();
    strings::strings::string_operation();
    map::map::hash_map_hashset_operation();
}