use std::collections::HashMap;
use std::{char};
use std::{ collections::{HashSet}};
mod numbers;
mod array;
mod strings;
mod map;
//Problem Statement







// //ferceny in the eh charactor
// fn frequency_ch(s : &str)-> HashMap<char , i32>{
//     let mut num = HashMap::new();
//     for i in s.chars(){
//         *num.entry(i).or_insert(0) += 1 
//     }
//     num
// }
// //prime numbers 
// fn prime_numbers(s : i32)-> bool{
//    if 2 < 0 {
//     return  false;
//     }
//     let limit = (s as f64).sqrt() as i32;
//     for i in 2..=limit{
//         if 2 % i == 0 {
//          return  false;
//         }
//     }
//     true


// }






// //This is devide both number exactly with out number 
// fn gcd_numbers( mut a : i32 , mut b : i32)-> i32{
//    if b == 0 {
//     return a;

//    }else if a == 0 {
//        return b;
//    }
//    while b != 0  {
//        let temp = a % b;
//        a = b ;
//        b = temp
//    }
// a
// }

// fn lcm_numbers(  a : i32 ,  b : i32)-> i32{
//     let gcd_value = gcd_numbers(a, b);
     
     
//         let temp = (a * b ) / gcd_value;
       
     
//      temp
     
// }






// //divide words
// fn divide_words(s : &str)-> HashMap<String ,i32>{
//         let mut sum = HashMap::new();
//         for i in s.split_whitespace(){
//           *sum.entry(i.to_string()).or_insert(0) += 1
          
//         }
//        sum
// }
// //palagram 
// fn palagram_word(s : &str)-> HashSet<char>{
//     let mut alp = HashSet::new();
//     for i in s.chars(){
//         alp.insert(i);
//     }
//     alp
// }
// //Remove The duplicate the array
// fn remove_duplicate(s :Vec<char>)-> HashSet<char>{
//    let mut  sum = HashSet::new();
//      for i in s{
//         sum.insert(i);
//      }
//      sum
// }
// fn incremnet(s: &str)->Vec<(String , usize)>{
//    let words: Vec<String> = s.split_whitespace().map(|s|s.to_string()).collect();
//    let mut result = Vec::new();
//    for (word,i) in words.iter().enumerate(){
//       result.push((i.clone(), word + 1));
//    }
//    result
// }

// //count each numr 
// fn each_umber(s : &str)-> HashMap<String , i32>{
//    let mut count = HashMap::new();
//    for i in s.split_whitespace(){
//     *count.entry(i.to_string()).or_insert(0) += 1
//    }
//    count
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
// //The pergect numebr is devided ;
// fn is_perfect(s : i32)-> bool {
//     let mut sum = 0;
//     for i in 1..=s/2 {
//       if s % i == 0 {
//        sum += i
//       }
//     }
//    sum == s
// }


// //sum of digits 
// fn sum_of_digits(s : i32)-> i32{
//   let mut sum = 0;
//   for i in s.to_string().chars(){
//      sum += i.to_digit(10).unwrap() as i32;

//   }
//   sum
// }
// fn revrese_strings(s1 : &str , s2 : Vec<i32>)-> Vec<(String , i32)>{
//    let result = s1.chars().rev().clone().collect::<String>();
//    let result1 = s2.iter().rev().copied().collect::<Vec<i32>>();
//    result1.iter().map(|x|(result.clone(), *x )).collect()
// }




// //Revrse number 
// fn reverse_number_oreder(s : Vec<i32>)-> Vec<i32>{
//    let mut num = s.clone();
//    for pass in 0..num.len(){
//     for i in 0..num.len() - pass - 1 {
//       if num[i] > num [i + 1]{
//         num.swap(i, i + 1);
//       }
//     }
//    }
//    num
// }

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
// //Word frimary is 
// fn lowercase_words(s : &str)-> HashMap<String , i32>{
//     let mut str = HashMap::new();
//     for i in s.split_whitespace(){
//        let clear = i.chars().filter(|v | v.is_alphanumeric()).collect::<String>().to_uppercase();
//        str.entry(clear).and_modify(|v|*v += 1).or_insert(1);
//     }
//     str
// }
// //Sort string by lenght 
// fn ordering_string_by_length(s : Vec<String>)-> Vec<String>{
//    let mut str = s.clone();
//    for pass in 0..str.len(){
//    for i in 0..str.len() - pass - 1{
//        if str[i].len() > str[i + 1 ].len(){
//            str.swap(i, i + 1);
//        }
//    }
//    }
//    str
// }
// // //Non repeating string is 
// // fn non_repeating_strig(s : &str)->Option<char>{
// //     let mut str = HashMap::new();
// //     for i in s.chars(){
// //       *str.entry(i).or_insert(0) += 1;
// //     }
// //     for i in s.chars(){
// //       if let Some(&count) = str.get(&i){
// //         if count == 1 {
// //           return Some(i);
// //         }
// //       }
// //     }
// //     None
// // }

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
// //PrimeNumbers 
// fn reverse_same_number(s : i32)-> bool{
//    let num = s.to_string();
  
//     let  rev = num.chars().rev().collect::<String>();
//       if rev == num {
//            return true
//       } else {
//           return false;    
//       }
//    }
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

// //Arry palindrome NUMBER 120 TO 150
// fn array_palindrome_numbers(s: i32, s1: i32) -> Vec<i32> {
//     let mut num = Vec::new();
//     for d in s..=s1 {
//         let rev = d.to_string();
//         let rev1 = rev.chars().rev().collect::<String>();

//         if rev == rev1 {
//             num.push(d);
//         }
//     }
//     num
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

// //Reverse word string
// //"Hello Word" -> Expected output Word Hello

// fn revrse_word (s : &str)-> String{
//       let  mut str = Vec::new();
//       for i in s.split_whitespace(){
//        let rev = i.chars().rev().collect::<String>(  );
//          str.push(rev);
          
//       }
//       str.join(" ")
// }

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
  //  let a  = 102;
  //  let b = 104; 
  //  let num = 4;
  //  let  number = vec![1 ,2 , 3 , 4, 5, 6 , 7 , 8 , 9, 10];
  //  let sentense = "ghe manoj seetaram ";
  //  let palagram = "Hey im a manoj seetaram system programmer in rust thank you manoj manoj";
  //  let duplicate = vec!['a' ,'a' ,'b' , 'c' , 'd' , 'e' , 'f' , 'g' , 'k' , 'k'];
  //  let word = "leve";
  //   let anagram = "silen";
  //   let anagram1 = "lisen";
  //   let num8 = 0;
  //   let aray = vec![1 ,3 , 4 ,2, 5, 9 , 6 , 8 ,7];
  //     let aray1 = vec![1 ,3 , 4 ,2, 5, 9 , 6 , 8 ,7];
  //   let  duplicates = vec![1,2,3,4,5 , 6, 5,];
  //  let digit = 1234;
  //   let start1 = 1 ;
  // let end1 = 20;
  // let num3 = 121;
  // let rev1 = 120 ;
  // let rev2 = 150;
  // let fact = 14;
  // let strong  = 145;
   
   
    // let n = 9;
  //   let n1 = 1234;
   
  // let s = "babad";
  // let caps = "hello manoj seetaram";
    
  // let reverse = "Hello WordE";
  //   let s2 = "cbbd";
  //  let result = revrse_word(reverse);
  //  let order = vec!["Hello".to_string() , "Manoj".to_string() , "Seetaram".to_string() , "How".to_string()];
    //   println!("The Gcd number is :{:?} , {:?}", gcd_numbers(a, b))  
    // let none_repeat = "manoj seetaram";
    numbers::numbers::numbers_operatins();
    array::array::array_operation();
    strings::strings::string_operation();
    map::map::hash_map_hashset_operation();
//     println!("The Gcd number is : {}", gcd_numbers(a.clone(), b.clone()));
//     println!("The Lcm number is : {}" , lcm_numbers(a.clone(), b.clone()));
//     println!("The number : {} and eqls : {}" ,num ,arm_strong(num) );
   
//     println!("The sentense is : {:?}" ,divide_words(sentense));
//     println!("The Alphabetic words : {:?}" , palagram_word(palagram));
//     println!("The remove duplicate the array : {:?}", remove_duplicate(duplicate));
//     println!("The for each word increment the count  :{:?}" , incremnet(palagram));
//     println!("The count each number : {:?}", each_umber(palagram));
//     println!("Thr revrs word is : {}" , reverse_palindrome(word));
//     println!("Rmove the palinfrome : {:?}", remove_palinfrome(word));
//     println!("The anagram word is ; {} -> {} -> {}" ,anagram , anagram1,   anagram_cheker(anagram , anagram1));
   
//     println!("The missing numbers : {:?}" , missing_number(aray));
//     println!("The remove dupicat number : {:?}", duplicate_number(duplicates.clone()));
//     println!("The longest word in sentense : {}" , longest_word(palagram));
//     println!("The number is perfect : {}" , is_perfect(num));
//     // println!("The factorial number is : {} ", factorial_numbers(&num));
//     // println!("The Fibonaci {} number is : {}" ,num , fibbonaci_number(num) );
//     println!("The sum of digits for last number : {:?}" ,sum_of_digits(digit));
//    //  println!("The reverse {:?}  string : {:?} ->{:?}" ,anagram , duplicates ,revrese_string(anagram , duplicates));
//     println!("The reverse {:?} {:?} string: {:?}", anagram, duplicates, revrese_strings(anagram, duplicates.clone()));

// // println!("The smalest number : {}" , smalest_numbers(number.clone()));
// println!("The order wise numbers : {:?}" ,order_numbers(aray1.clone()));
// // println!("The Factorial number : {:?}" ,factorial_numberss(&num.clone()) );
// println!("The oreder number : {:?} " , reverse_number_oreder(aray1.clone()));
// println!("The order string setting : {:?}" ,order_string(order.clone()));
// println!("The Non repeating charactor is : {:?}" ,non_repeating(sentense));
// println!("The Sort the string by. length : {:?}" ,ordering_string_by_length(order.clone()) );
// // println!("The Non repeating elements in string is : {:#?} - > {:?} - > {:?}" ,non_repeating_strig(sentense.clone(), none_repeat.clone() ,palagram.clone()));

// println!("The word frequency is : {:?}",word_frequncy(palagram));
// println!("The word frequency is : {:?}" ,lowercase_words(palagram) );
// println!("Remove the duplicate number : {:?}" , duplicate_in_array(duplicates.clone()));
//  println!("This is a prime numbers :{:?} " , prime_numberss(start1 , end1));
//   println!("This is a palinfrome numbers : {:?}",reverse_same_number(num3));

// //   println!("This is a Araay palindome numbers : {:?}", array_palindrome_number(rev1 , rev2));
//   println!("The strong number is true or false : {:?}",strong_number(strong));
//   println!("The factorial number : {:?}", factorial_number(&fact));
//   println!("The is array palindrome number : {:?}" , array_palindrome_numbers(rev1.clone() , rev2.clone()));
//   println!("Longest Palindrome in '{}' is '{}'", s, longest_palindrome(s));
//   println!("Longest Palindrome in '{}' is '{}'", s2, longest_palindrome(s2));
//   println!("The reverse string : {}" ,result);
//   println!("The vovels : {:?}" , vowels_in_string(reverse));
//   println!("The First word in capiatal : {:?}" , first_word_capital(caps));
     

    
   


   
    
//      println!("The frequency in the each charactor : {:?}" ,frequency_ch(str));
//      for &numbers in num4.iter(){
   
//      println!("The prime {:?} ? numbers {:?}",numbers, prime_numbers(numbers))
//      }
//      println!("This is a sum : {:?}" , sum_digits(n1))



}

