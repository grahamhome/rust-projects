use std::io;

fn main() {
    loop {
        println!("Enter a temperature in Fahrenheit to convert or 'exit' to exit.");
        let mut entry = String::new();
        io::stdin()
            .read_line(&mut entry)
            .expect("Failed to read entry.");
        if entry.trim() == "exit" {
            break;
        }
        let entry: f32 = match entry.trim().parse() {
            Ok(temp) => temp,
            Err(_) => continue,
        };
        let fact = 5.0/9.0;
        println!("Conversion factor is {fact}");
        let celsius = (entry - 32.0) * (5.0/9.0);
        println!("{entry} degrees Fahrenheit is {celsius}")
    }
}
