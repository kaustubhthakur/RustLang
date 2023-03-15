/*Closures are functions that can capture the enclosing environment. For example, a closure that captures the x variable: */

fn main(){
let ans = || 1;
println!("closure return one :{}",ans());

}