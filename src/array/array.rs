
//1 .revrese number 
pub fn revrse_num(s : Vec<char>)-> String{
    s.iter().rev().collect::<String>()
}
//2.Find the largest number
pub fn largest_number(s : Vec<i32>)-> i32{
    let mut num = s[0];
    for i in s.iter(){
      if *i > num {
        num = *i
      }
    }
    num
} 
pub fn array_operation(){
     let num = vec!['1' , '2' ,'3' , '4' , '5','6'];
     let num_op = vec![1 ,2 ,3 ,4 , 5 ,6];


     println!("This is revrse number : {:#?}" ,revrse_num(num));
     println!("The largest number is : {}", largest_number(num_op));
     

}