//Calculte the ntg Fibbonaci numbers 
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
//fibonaci
pub fn fibonaci(s : i32)-> i32{
    let (mut a ,mut b ) = (0 , 1);
    for _ in 0..s{
     let next = a + b ;
     a = b;
     b = next
    }
a
}  
pub fn numbers_operatins(){
    let n = 8;
    
    println!("The {}th Fibonacci number is: {}", n, ntg_fibonaci(n));
    println!("The {}th Fibonacci number is: {}", n,fibonaci(n));
}