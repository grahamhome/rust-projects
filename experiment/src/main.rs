fn sum(values: &[i32]) -> i32 {
    let mut result = 0;
    for i in 0..values.len() {
        result += values[i];
    }
    result
}

fn main() {
    let array = [10,20,30,40,50];
    let result = sum(&array);
    println!("Sum: {}", result);
}
