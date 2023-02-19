//it is collection of object
//it is created using brackets

fn main()
{

    //fixed array
    let xs: [i32; 5] = [1, 2, 89898, 4, 5];

    for i in 0..xs.len(){
        println!("the elements in array are {}",xs[i]);
    }

}