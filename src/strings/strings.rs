

//1 .revrse string
use std::collections::HashMap;
pub fn revrese_string(s :&str)-> String{
     let mut str = String::new();
     for i in s.chars(){
        str.insert(0, i);
     }
     str
}
//2. Vovels aieou
pub fn vovels(s : &str)-> String{
    let mut count = 0;
    for i in s.chars(){
        let ch = i.to_lowercase().next().unwrap();
        if ch == 'a' || ch == 'e' || ch == 'o' || ch == 'i' || ch == 'u' {
            count += 1
        }
    }
    count.to_string()

}
//3.order by the string length 
pub fn order_string_length(s : Vec<String>)-> Vec<String>{
      let mut str = s.clone();
      for pass in 0..str.len(){
        for i in 0..str.len() - pass - 1 {
       if str[i].len() > s[i + 1].len(){
           str.swap(i, i + 1);
       }
        }
      }
      str
}
// 4. count the word 
pub fn count_words_in_string(s : &str)-> HashMap<String , i32>{
    let mut count = HashMap::new();
    for i in s.split_whitespace(){
        *count.entry(i.to_string()).or_insert(0) += 1;
    }
    count

}
//5. reverse string and number 
pub fn revrse_string_and_number(s : &str , s1 : Vec<i32>)-> Vec<(String , i32)>{
     let str = s.chars().rev().clone().collect::<String>();
     let num = s1.iter().rev().copied().collect::<Vec<i32>>();
     num.iter().map(|v|(str.clone(), *v)).collect()
}
//6. reverse word not a full string like "Memory sefty" -> "ryomeM ytfrs"
pub fn revrse_word(s : &str)-> String{
      let mut word = Vec::new();
      for i in s.split_whitespace(){
       let str = i.chars().rev().collect::<String>();
       word.push(str);
      }
      word.join(" ")
}
//7. Non repeating strings exp : seetaram -> stm
pub fn non_repeating_strig(s : &str)-> Option<char>{
  let mut str = HashMap::new();
  for i in s.chars(){
      *str.entry(i).or_insert(0) += 1;
  }
  for i in s.chars(){
    if let Some(&count) = str.get(&i){
       if count == 1 {
        return Some(i);
       }
    }
  }
  None
}
// 8 . Starting word capiatal
pub fn first_word_uppercase(s : &str)-> String{
  let mut word = Vec::new();
  for i in s.split_whitespace(){
    let mut ch = i.chars();
    let up = ch.next().map(|v|v.to_ascii_uppercase());
    let low = ch.skip(1).map(|v|v.to_ascii_lowercase()).collect::<String>();
    let words = match up {
        Some(v) => format!("{}{}", v , low),
        None => String::new(),
    };
    word.push(words);
  }
  word.join("")
}
// 9. Increment the count in string words 
pub fn increment_the_count(s : &str)-> Vec<(String , usize)>{
  let words : Vec<String> = s.split_whitespace().map(|s|s.to_string()).collect();
  let mut result = Vec::new();
  for (i , word) in words.iter().enumerate(){
       result.push((word.clone() , i + 1));
  }
  result
}
// 10. lonest word in the string 
pub fn lonest_word_in_the_string(s : &str)->String{
    let mut longest = String::new();
    for i in s.split_whitespace(){
      if i.len() > longest.len(){
        longest = i.to_string();
      }
    }
   longest
}
//11 . Anagram checkr silent -> Listen = true hello - word = false
pub fn anagram_checker(s : &str , s1 : &str)-> bool{
   let mut result = s.chars().collect::<Vec<_>>();
   let mut result1 = s1.chars().collect::<Vec<_>>();
   result.sort();
   result1.sort();
   result == result1

}
// 12.Find the longest palindromic substring
fn longest_palindrome(s : &str)-> String{
  
   
   let str : Vec<char> = s.chars().collect();
   let mut best_start = 0 ;
   let mut best_len = 0; 
   fn expand_around(chars : &Vec<char> ,mut left : isize ,mut right : isize)->(usize , usize){
      while left >= 0 
         && right < chars.len() as isize
         && chars[left as usize] == chars[right as usize]
         {
          left -= 1 ;
          right += 1
          }
    ((left + 1) as usize , (right - 1 ) as usize)
   }
   for i in 0..str.len(){
      let (l , r) = expand_around(&str,i as isize, i as isize);
      if r - i + 1 > best_len{
         best_start = l ;
         best_len = r - l + 1;

      }
      let (l1 , r1) = expand_around(&str, i as isize, i as isize + 1);
      if r1 >= l1 && r1 - l1 + 1 > best_len{
         best_start = l1;
         best_len = r1 - l1 + 1;
      }
   }

str[best_start..best_start + best_len].iter().collect()


   }

pub fn string_operation(){
    let str = "Manojseetaram";
    let word = "I love Rust , Rust is not cult";
    let num = vec![1 , 2 ,3 ,4 ,5 ,6 ,7];
    let s = vec!["apple".to_string() , "kiwi".to_string() , "banana".to_string()];
    let anagram = "silent";
    let anagram1 = "listen";
    println!("This is reverse string : {:?}" ,revrese_string(str));
    println!("Vovels in string : {}",vovels(str) );
    println!("Order by the string lenth : {:?}",order_string_length(s));
    println!("Count the word : {:?}" , count_words_in_string(word));
    println!("Reverse string and number : {:?}", revrse_string_and_number(word , num));
    println!("Rverse the Word : {} -> {:?}", word , revrse_word(word));
    println!("Non repeating char : {} -> {:?}",word,non_repeating_strig(word));
    println!("Staring word capiatal : {} = {}  ", word , first_word_uppercase(word));
    println!("Incremnet the count in string words : {:?} ",increment_the_count(word));
    println!("Longest word in the string : {} = {}",word ,lonest_word_in_the_string(word) );
    println!("Anagarm checker : {:?}",anagram_checker(anagram,anagram1) );
    println!("Lonest palindrome : {:?}", longest_palindrome(str));

}