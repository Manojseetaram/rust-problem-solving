use std::collections::HashMap;

//1. count the number of charactor 
fn count_chars(s: &str)-> HashMap<char , i32>{
  let mut count = HashMap::new();
  for i in s.chars(){
    *count.entry(i).or_insert(1) += 1;
  }
  count
}




pub fn hash_map_hashset_operation(){
    let str = "Manoj seetaram";
    println!("Countin the number of charactors : {:?} ",count_chars(str))
}