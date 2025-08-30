

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
pub fn string_operation(){
    let str = "Manojseetaram";
    let word = "I love Rust , Rust is not cult";
    let num = vec![1 , 2 ,3 ,4 ,5 ,6 ,7];
    let s = vec!["apple".to_string() , "kiwi".to_string() , "banana".to_string()];
    println!("This is reverse string : {:?}" ,revrese_string(str));
    println!("Vovels in string : {}",vovels(str) );
    println!("Order by the string lenth : {:?}",order_string_length(s));
    println!("Count the word : {:?}" , count_words_in_string(word));
    println!("Reverse string and number : {:?}", revrse_string_and_number(word , num));
    println!("Rverse the Word : {} -> {:?}", word , revrse_word(word));
    println!("Non repeating strings : {} -> {:?}",word,non_repeating_strig(word))
}