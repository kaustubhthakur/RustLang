// struct
struct User 
{
    active:bool,
    username:String,
    email:String,
    cnt:u64,
}
fn main() 
{
   let user = User {
    active:true,
    username:String::from("kaustubhthakur1234"),
    email:String::from("alexthegreat@1234"),
    cnt:2,
   };
   println!(" username  {} -> cnt {} ", user.username, user.cnt );
}
