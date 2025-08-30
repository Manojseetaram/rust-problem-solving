

//1 .revrse string

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
pub fn string_operation(){
    let str = "Manojseetaram";
    let s = vec!["apple".to_string() , "kiwi".to_string() , "banana".to_string()];
    println!("This is reverse string : {:?}" ,revrese_string(str));
    println!("Vovels in string : {}",vovels(str) );
    println!("Order by the string lenth : {:?}",order_string_length(s))
}