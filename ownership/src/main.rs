
fn main() 
{
    let s1 = String::from("helloworld");
    let (s2,len) = calc(s1);
    println!("The length of '{}' is {}.", s2, len);
}
fn calc(s:String) -> (String ,usize)
{
 let length = s.len();
 (s,length)   
}