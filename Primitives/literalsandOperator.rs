//scalar type
/*
signed integers: i8, i16, i32, i64, i128 and isize (pointer size)
unsigned integers: u8, u16, u32, u64, u128 and usize (pointer size)
floating point: f32, f64
char Unicode scalar values like 'a', 'α' and '∞' (4 bytes each)
bool either true or false
and the unit type (), whose only possible value is an empty tuple: ()

*/
//compound types
/*
arrays like [1, 2, 3]
tuples like (1, true)
*/
fn main()
{
    //mutable value can be change;
    let mut mutable = 12;
    let logical :bool = true;
    let a_float :f64 = 1.0;
    let an_integer = 5i32;
    println!("{} {} {} {}",mutable,logical,a_float,an_integer);
}