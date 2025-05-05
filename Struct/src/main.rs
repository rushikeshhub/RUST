fn main() {
    println!("This  is struct example");
    
    // Creating stuct

    struct User {
        id: i32,
        name: String,
        email: String,
        active: bool,
    }

  let mut user1 = User {
      id: 1,
      name: String::from("rushikesh"),
      email: String::from("abc@gmail.com"),
      active:true,
  };

  println!("details of user {0}",user1.name);

}
