

//2 .revrse string

pub fn revrese_string(s :&str)-> String{
     let mut str = String::new();
     for i in s.chars(){
        str.insert(0, i);
     }
     str
}
pub fn string_operation(){
    let str = "Manojseetaram";
    println!("This is reverse string : {:?}" ,revrese_string(str));
}