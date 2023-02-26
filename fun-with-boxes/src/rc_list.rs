use std::rc::Rc;

/// A list that can share ownership of sub-lists with other lists.
enum List {
    Cons(i32, Rc<List>),
    Nil
}

#[cfg(test)]
mod tests {
    use crate::rc_list::List::{Cons, Nil};
    use std::rc::Rc;
    #[test]
    fn use_list() {
        // Build list
        let a = Rc::new(Cons(5, Rc::new(Cons(6, Rc::new(Nil)))));
        println!("'a' reference count after creating 'a': {}", Rc::strong_count(&a));
        // Rc::clone() simply increments reference counter for value, does not deep copy value.
        let b = Cons(4, Rc::clone(&a));
        println!("'a' reference count after creating 'b': {}", Rc::strong_count(&a));
        let c = Cons(3, Rc::clone(&a));
        println!("'a' reference count after creating 'c': {}", Rc::strong_count(&a));
        drop(b);
        println!("'a' reference count after dropping 'b': {}", Rc::strong_count(&a));
    }
}