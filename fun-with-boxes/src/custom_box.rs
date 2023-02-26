use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    // Just gives a reference, MyBox maintains ownership of value
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {name}")
}

#[cfg(test)]
mod tests {
    use crate::custom_box::{MyBox, hello};
    #[test]
    fn dereference() {
        let x = 5;
        let y = MyBox::new(x);
        assert_eq!(x, 5);
        assert_eq!(*y, x);

        let m = MyBox::new(String::from("Rustacean"));

        hello(&m);

        // Deref coercion makes this equivalent to:
        hello(&(*m)[..]);

        // But Rust calls .deref() as many times as needed to get a reference that matches
        // the parameter's type.
    }
}
