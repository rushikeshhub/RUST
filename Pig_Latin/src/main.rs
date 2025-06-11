use std::io;

fn main() {
    println!("Hello, world!, we will creating Pig Latin string ");
    println!("Please enter the String :");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut pig_latin_word = Vec::new();
    
    for word in input.split_whitespace(){
       let vowels = ['a','e','i','o','u'];
       let first_char = word.chars().next().unwrap();
       let new_word = if vowels.contains(&first_char.to_ascii_lowercase()) {
        format!("{}-hay",word)
       }else {
        let rest:String=word.chars().skip(1).collect();
        format!("{}-{}ay",rest,first_char)
       };
       pig_latin_word.push(new_word);
    };

    println!("Pig Latin Word : {}",pig_latin_word.join(" "));
}
