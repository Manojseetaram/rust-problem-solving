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
pub fn fibonaci(n : i32)-> i32{
    let (mut a   , mut b ) = (0 , 1);
    if n == 0 {
        return  a;
    }
    if n == 1 {
        return  b;
    }
    for _ in 2..= n {
        let next = a  + b ;
        a = b ;
        b = next
    }
    b
}   
pub fn numbers_operatins(){
    let n = 8;
    println!("The {}th Fibonacci number is: {}", n, ntg_fibonaci(n));
    println!("The {}th Fibonacci number is: {}", n,fibonaci(n));
}