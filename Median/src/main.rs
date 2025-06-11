
use std::io;
use std::collections::HashMap;


fn main()
{
    println!("Hello, welcome to program");
    let mut a = String::new();
    println!("Pleae enter how many integer you wann to store");
    
    io::stdin()
    .read_line(&mut a)
    .expect("Failed to read line");
     
     let count:usize = match a.trim().parse() {
        Ok(num) => num, 
        Err(_) => {
            println!("Please enter Valid number ");
            return;
        }
        
     };
        
    let mut v:Vec<i32> = Vec::new();


    while v.len() < count {

    println!("Pleae enter next integer you wann to store");
    let mut b = String::new();
    io::stdin()
    .read_line(&mut b)
    .expect("Failed to read line");
     
     match b.trim().parse() {
        Ok(num) => v.push(num), 
        Err(_) => println!("Please enter Valid number "),
     };
     
    }

    println!(" these are list of number you have entered .");
    v.sort();
    
    for i in &v {
        println!{"{}",i};
    }

    let median = if v.len() % 2 == 1{
        v[v.len() / 2] as f64
        
    }else {
        let mid = v.len()/2;
        (v[mid -1] + v[mid]) as f64 / 2.0
    };

    println!("the median is {}",median);

    let mut h = HashMap::new();

    for &x in &v{
        *h.entry(x).or_insert(0) += 1;
    }
     let mode = h.iter()
     .max_by_key(|entry|entry.1)
     .map(|(key, _)| *key)
     .unwrap();

    println!{"Mode is :{}",mode};
  

}
