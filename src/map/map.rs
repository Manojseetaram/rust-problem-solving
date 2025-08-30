use std::collections::HashMap;

//1. count the number of charactor 
pub fn count_chars(s: &str)-> HashMap<char , i32>{
  let mut count = HashMap::new();
  for i in s.chars(){
    *count.entry(i).or_insert(1) += 1;
  }
  count
}
//2. count the string number and also convert to the uppercase 
pub fn uppercase_string(s : &str)-> HashMap<String , i32>{
    let mut str = HashMap::new();
    for i in s.split_whitespace(){
        let clear = i.chars().filter(|v|v.is_alphanumeric()).collect::<String>().to_uppercase();
          str.entry(clear).and_modify(|v|*v += 1 ).or_insert(1);
    }
    str
}


pub fn hash_map_hashset_operation(){
    let str = "Manoj seetaram";
    println!("Countin the number of charactors : {:?} ",count_chars(str));
    println!("Lower case converted to the uppercase : {:?}",uppercase_string(str))

}