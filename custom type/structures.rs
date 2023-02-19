/*

There are three types of structures ("structs") that can be created using the struct keyword:

Tuple structs, which are, basically, named tuples.
The classic C structs
Unit structs, which are field-less, are useful for generics.


*/
#![allow(dead_code)]

#[derive(Debug)]

struct Person 
{
    name:String,
    age :u8,
}
fn main()
{
    let name = String::from("alex");
    let age = 19;  
let alex = Person{name,age};
println!("{:?}",alex);

}