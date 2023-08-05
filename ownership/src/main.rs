/*
fn main() 
{
    let s1 = String::from("hello world");
    let (s2,len) = calc(s1);
    println!("The length of '{}' is {}.", s2, len);
}
fn calc(s:String) -> (String ,usize)
{
 let length = s.len();
 (s,length)   
}
*/
//ownership and fucntion 

/*
fn main() 
{
    let s = String::from("kaustubh");
   take_ownership(s);
   let x = 69;
   make(x);
}
fn take_ownership(str:String)
{
    println!("{}",str);
}
fn make(n:i32)
{
    println!("{}",n);
}
*/
/*
fn main()
{
    let dx = give_ownership();
    let dy = String ::from("hello kaustubh");
    let dz = takes(dy);
}
fn give_ownership() -> String 
{
    let dxx = String::from("yours");
    dxx;
}
fn takes(str:String) -> String{
str;
}
*/