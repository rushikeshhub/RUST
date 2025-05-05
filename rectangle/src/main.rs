#[derive(Debug)]


struct rectangle{
    height: i32,
    weidth: i32
}


fn area(r: &rectangle) -> i32 {
    r.height*r.weidth
}


fn main() {
    println!("Hello, world!, we are calculating area of Rectangle");

    let r= rectangle{
        height: 30,
        weidth: 30
    };

    let a=area(&r);
    println!("Area of rectangle is {}",a);
}
