#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two(x: u32) -> u32 {
    x + 2
}

fn greeting(input: &str) -> String {
    format!("Hello {}!", input)
}

pub struct Guess {
    value: i32
}

impl Guess {
    fn new(value: i32) -> Guess {
        if value > 100 || value < 1 {
            panic!("Guess value must be between 1 and 100, got {}", value)
        }
        Guess {
            value
        }
    }
}

fn print_and_return_value(value: u32) -> u32 {
    println!("I got the value {}", value);
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn larger_holds_smaller() {
        let smaller_rect = Rectangle {
            width: 32,
            height: 44,
        };
        let larger_rect = Rectangle {
            width: 33,
            height: 45,
        };
        assert!(larger_rect.can_hold(&smaller_rect))
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let smaller_rect = Rectangle {
            height: 3,
            width: 4,
        };
        let larger_rect = Rectangle {
            height: 2,
            width: 4,
        };
        assert!(!smaller_rect.can_hold(&larger_rect))
    }

    #[test]
    fn two_plus_two() {
        assert_eq!(add_two(2), 4)
    }

    #[test]
    fn greeting_contains_name() {
        let name = "Graham";
        assert!(greeting(name).contains(name))
    }

    #[test]
    #[should_panic(expected = "Guess value must be between 1 and 100")]
    fn guess_panics() {
        Guess::new(101);
    }

    #[test]
    fn it_works_2() -> Result<(), String> {
        if add_two(2) == 4 {
            Ok(())
        } else {
            Err(String::from("Two plus two should equal 4!"))
        }
    }

    #[test]
    fn it_prints_and_returns() {
        assert_eq!(print_and_return_value(10), 10)
    }
}
