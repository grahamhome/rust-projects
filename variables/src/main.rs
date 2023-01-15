fn main() {
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");

    let message = "The temperature today is:";
    let x = [message; 100];
    println!("{} {}", x[0], x[1]);
}
