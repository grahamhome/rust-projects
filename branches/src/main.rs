fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let condition;
    condition = true;
    let number = if condition {5} else {6};

    println!("Value: {number}");

    let x = true;
    let y = if x {};

    println!("y is {y:?}")
}
