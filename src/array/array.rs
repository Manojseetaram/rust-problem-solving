
//1 .revrese number 
pub fn revrse_num(s : Vec<char>)-> String{
    s.iter().rev().collect::<String>()
}
pub fn array_operation(){
     let num = vec!['1' , '2' ,'3' , '4' , '5','6'];
   println!("This is revrse number : {:#?}" ,revrse_num(num));
}