

use std::{ collections::{HashSet}};
mod numbers;
mod array;
mod strings;
mod map;

























// //palagram 
// fn palagram_word(s : &str)-> HashSet<char>{
//     let mut alp = HashSet::new();
//     for i in s.chars(){
//         alp.insert(i);
//     }
//     alp
// }

// fn incremnet(s: &str)->Vec<(String , usize)>{
//    let words: Vec<String> = s.split_whitespace().map(|s|s.to_string()).collect();
//    let mut result = Vec::new();
//    for (word,i) in words.iter().enumerate(){
//       result.push((i.clone(), word + 1));
//    }
//    result
// }


// //reverse also same word  palindrome checker 
// fn reverse_palindrome(s : &str)-> bool{
 
//       let reverse = s.chars().rev().collect::<String>();
//       reverse == s
      
     
// }

// //Remove alpabets

// fn remove_palinfrome(s: &str)-> HashSet<char>{
//      let mut count = HashSet::new();

//      for i in s.chars(){
//         count.insert(i);
//      }
//      count
// }
// fn anagram_cheker( s1:&str , s2: &str)-> bool{
//   let mut result = s1.chars().collect::<Vec<_>>();
//   let mut result1 = s2.chars().collect::<Vec<_>>();
//   result.sort();
//   result1.sort();
//   result1 == result
     
// }


// //mising array number
// fn missing_number(s : Vec<i32>)-> i32{
//    let num = s.len() as i32 + 1;
//    let arra = num * (num + 1) / 2 ;
//    let sums : i32 = s.iter().sum();
//    arra - sums
// }
// //Find the duplicate in array 
// fn duplicate_number(s: Vec<i32>)-> HashSet<i32>{
//   let mut num = HashSet::new();

//   for i in s.iter(){
//     num.insert(*i);
//   }
//   num
// }
// //Longest word in a sentents
// fn longest_word(s : &str)-> String{
//     let mut longest = String::new();

//     for i in s.split_whitespace(){
//       if i.len() > longest.len(){
//         longest = i.to_string()
//       }
//     }
//     longest
// }
//







// //Find non repeating character fn non_repeating(s : &str)-> Option<char>{
// fn non_repeating(s : &str)-> Option<char>{
//   //Store the string in HashMap
//     let mut str = HashMap::new();
//     //count the all string and take the charactore number 
//     for i in s.chars(){
//       *str.entry(i).or_insert(0) += 1 ;
//     }
//     for i in s.chars(){
//       if let Some(&count) = str.get(&i){
//         if count == 1 {
//           return  Some(i);
//         }
//       }
//     }
  
//     None
   
    
// }


// /
// //word priquency is 
// fn word_frequncy(s : &str)-> HashMap<String , i32>{
//     let mut str = HashMap::new();
//     for i in s.split_whitespace(){
//        let clean = i.chars().filter(|c | c.is_alphanumeric()).collect::<String>().to_lowercase();
//        str.entry(clean).and_modify(|v|*v += 1).or_insert(1);
//     }
//     str
// }
// //count duplicate in arraya
// fn duplicate_in_array(s : Vec<i32>)-> HashMap<i32 , i32>{
//     let mut num = HashMap::new();

//     for i in 0..s.len(){
    
//         *num.entry(s[i]).or_insert(0) += 1;
      
//     }
//     num
// }
// fn prime_numberss(s : i32 , s1 : i32 )-> Vec<i32>{
//   let mut num = Vec::new();
//   for d in 2..=s.into(){
//    let limit = (d as f64).sqrt() as i32;
//    let mut is_num = true;
//    for i in 2..=limit{
//       if d % i == 0 {
//          is_num = false;
//          break;
//       }
//    }
//    if is_num {
//       num.push(d);
//    }
//   }
// for d in 2..=s1.into(){
//    let limit = (d as f64).sqrt() as i32;
//      let mut is_num = true;
//    for i in 2..=limit{
//     if d % i == 0 {
//         is_num = false;
//         break;
//       }
      
//    }
//    if is_num{
//          num.push(d);
//       }
// }

// num
// }

//    //Arraya palindrome numbers in 120 to 150 ;
// //    fn array_palindrome_number(s : i32 , s1 : i32)-> Vec<i32>{
// //    let mut num = Vec::new();
// //    for d in s..=s1.into(){
// //     let is_num = d.to_string();
// //     let rev = is_num.chars().rev().collect::<String>();
// //     if is_num == rev {
// //       num.push(d);
// //     }
    
// //   }

// //   num

// //   }




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

// //Reverse word string
// //"Hello Word" -> Expected output Word Hello



// //Starting word capiatal 
// //"hello manoj seetaram"
// //Tommarrow plan i devide the problem stamnet based on methos and anslys when whre i use this method this is my task tommmarrow
// fn first_word_capital(s : &str)-> String{
//          let mut words = Vec::new();
//          for i in s.split_whitespace(){
//             let mut ch = i.chars();
//           let word = ch.next().map(|v|v.to_ascii_uppercase());
//           let lower = ch.skip(1).map(|v|v.to_ascii_lowercase()).collect::<String>();
//         let wordss = match word {
//             Some(v) => format!("{}{}", v , lower),
//            None => String::new(),
//         };
//         words.push(wordss);
//          }
//       words.join(" ")
// }
 


fn main(){
   
    numbers::numbers::numbers_operatins();
    array::array::array_operation();
    strings::strings::string_operation();
    map::map::hash_map_hashset_operation();
}