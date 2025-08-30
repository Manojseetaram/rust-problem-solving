



//1 .Calculte the ntg Fibbonaci numbers 
pub fn ntg_fibonaci(n :i32)-> i32{
    let (mut  a  , mut b ) = (0 , 1);
    if n == 0 {
        return  a;
    }
    else if  n == 1 {
        return b;
    }
    for _ in 2..=n{
       let next = a + b;
        a = b;
        b = next
    };
    b
}
//2.fibonaci
pub fn fibonaci(s : i32)-> i32{
    let (mut a ,mut b ) = (0 , 1);
    for _ in 0..s{
     let next = a + b ;
     a = b;
     b = next
    }
a
}  
//3.Factorial number
pub fn factorial_number(s : &i32)-> i32{
   let mut num = 1;
   for i in 1..=*s{
      if i == num {
        num *= i
      }
   }
   num
}
//4. Arrange order wise number
fn arrange_oreder_wise_number(s : Vec<i32>)-> Vec<i32>{
    let mut count = s.clone();
    for pass in 0..count.len(){
        for i in 0..count.len() - pass - 1 {
         if count[i] > count[i + 1]{
             count.swap(i, i + 1);
         }
        }
    }
    count
}
//5. sum of digit in the number 
pub fn  sum_of_number(s : i32)-> i32{
      let mut num = 0;
      for i in s.to_string().chars(){
       num += i.to_digit(10).unwrap() as i32
      }
      num
}
//6. Aarm strong numbers 
pub fn arm_strong_number(s : i32)-> bool{
   let mut sum = 0;
   let n = s.to_string().len() as u32;
   for i in s.to_string().chars(){
     let digit = i.to_digit(10).unwrap();
     sum += digit.pow(n)
   }
   sum == s.try_into().unwrap()
}
//7. Factorial of each number == sum number 
pub fn factorial_of_each_number(s : i32)-> bool{
    let  num = s.to_string();
    let mut sum = 0;
    for i in num.chars(){
      let n = i.to_digit(10).unwrap();
      let mut factorial = 1;
       for d in 1..=n{
        factorial *=d
       }
       sum += factorial
    }
    if sum == s.try_into().unwrap() {
        return true;
    }else {
        return false;
    }
}
pub fn numbers_operatins(){
    let n = 8;
    let number = vec![1 ,5, 8 , 9 , 4,2,3,6];
    let sum = 123;

    
    println!("The {}th Fibonacci number is: {}", n, ntg_fibonaci(n));
    println!("The {}th Fibonacci number is: {}", n,fibonaci(n));
    println!("The {}th Factorial number is: {}", n ,factorial_number(&n));
    println!("Arrange ordere wise number : {:?}", arrange_oreder_wise_number(number));
    println!("Sum of digit in the number : {}",sum_of_number(sum) );
    println!("The Arm strong number : {:?}" ,arm_strong_number(sum));
    println!("Factorial of each number == sum number {} = {}", sum ,factorial_of_each_number(sum) )
}