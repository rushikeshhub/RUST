//      Enum Feature       //
fn main(){
enum Data {
    Name(String),
    Age(i32),
    EmailId(String),
}

impl Data 
{
   fn call(&self) {
       match  self {
       Data::Name(name) => println!("Owner is {} ",name),
       Data::Age(age) => println!("age is {} ",age),
       Data::EmailId(email_id) => println!("email_id  is {} ",email_id),
   }
   }

}


    println!("Hello, world!");
    let n = Data::Name(String::from("Rushikesh Mhaske"));
    let a = Data::Age(26);
    let e = Data::EmailId(String::from("email"));
    n.call();
    a.call();
    e.call();
}
