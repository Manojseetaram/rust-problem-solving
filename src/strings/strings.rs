

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
pub fn string_operation(){
    let str = "Manojseetaram";
    println!("This is reverse string : {:?}" ,revrese_string(str));
    println!("Vovels in string : {}",vovels(str) )
}