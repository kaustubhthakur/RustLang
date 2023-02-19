//it is collection of values of different types
//it is constructed using ()

fn main()
{
    let tuples_a = (1u8, 2u16, 2u32 ,-2i16 );
    println!("tuple is {:?}",tuples_a);
    let tuple_a_tuple = (1u8,2u16,1u32,(-2i16,-3i16));
    println!("tuple of tuple is {:?}",tuple_a_tuple);

}