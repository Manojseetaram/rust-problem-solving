
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
//3. Find the smalest number 
pub fn smalest_number(s : Vec<i32>)-> i32{
  let mut num = s[0];
  for i in s.iter(){
    if *i < num {
        num = *i
    }
  }
  num
}
//4.Second largst number 
pub fn second_largest_number(s : Vec<i32>)-> i32{
  let mut num = s[0];
  let mut n = s[1];
  for i in s.iter(){
    if *i > num {
        let _ = n == num ;
        num = *i
    }else if *i > n {
        let _ = i < &num ;
        n = *i
    }
  }
  n
}
//5. second smalest number
pub fn second_smalest_number(s : Vec<i32>)-> i32{
     let mut num = s[0];
     let mut n = s[1];
     for i in s.iter(){
        if *i < num {
            let _ = num == n ;
           num = *i
        }else if *i < num {
            let _ = i > &num;
            n = *i
        }
     }
     n
}
//6. Missing number is 2 , 7
pub fn missing_number(s : Vec<i32>)-> i32{
 let  num = s.len() as i32 + 1;
 let  arra = num * (num + 1) / 2;
 let  sums : i32= s.iter().sum();
   arra - sums
}
//Array operation
pub fn array_operation(){
     let num = vec!['1' , '2' ,'3' , '4' , '5','6'];
     let num_op = vec![1 ,2 ,3 ,4 , 5 ,6];
     let number = vec![1 , 3 , 4 , 5, 6,8];


     println!("This is revrse number : {:#?}" ,revrse_num(num));
     println!("The largest number is : {}", largest_number(num_op.clone()));
     println!("The smalest number is : {}", smalest_number(num_op.clone()));
     println!("This is a Secondlargest number : {}",second_largest_number(num_op.clone()));
     println!("This is a Second smalest number : {}",second_smalest_number(num_op.clone()));
     println!("In this array the missing number is : {}", missing_number(number))
    


}