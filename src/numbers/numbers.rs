



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

pub fn numbers_operatins(){
    let n = 8;
    let number = vec![1 ,5, 8 , 9 , 4,2,3,6];

    
    println!("The {}th Fibonacci number is: {}", n, ntg_fibonaci(n));
    println!("The {}th Fibonacci number is: {}", n,fibonaci(n));
    println!("The {}th Factorial number is: {}", n ,factorial_number(&n));
    println!("Arrange ordere wise number : {:?}", arrange_oreder_wise_number(number))
}