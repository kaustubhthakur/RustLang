/*

Functions are declared using the fn keyword. Its arguments are type annotated, just like variables, and, if the function returns a value, the return type must be specified after an arrow ->.

*/


fn main()
{
   
    price(10);
}

fn price_ratio(call: u32) -> ()
{
   if call ==0 
   {
    return 0;
   }
   call+20;

}
fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    // Corner case, early return
    if rhs == 0 {
        return false;
    }

    // This is an expression, the `return` keyword is not necessary here
    lhs % rhs == 0
}
fn fizzbuzz(n: u32) -> () {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}
fn fizzbuzz_to(n: u32) {
    for n in 1..=n {
        fizzbuzz(n);
    }
}