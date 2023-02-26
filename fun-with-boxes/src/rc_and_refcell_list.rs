use std::rc::Rc;
use std::cell::RefCell;

/// A list that can share ownership of sub-lists with other lists.
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil
}

#[cfg(test)]
mod tests {
    use crate::rc_and_refcell_list::List::{Cons, Nil};
    use std::rc::Rc;
    use std::cell::RefCell;
    #[test]
    fn use_list() {
        let value = Rc::new(RefCell::new(5));

        let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

        let b = Cons(Rc::new(RefCell::new(5)), Rc::clone(&a));
        let c = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));

        *value.borrow_mut() += 10;

        println!("a = {:?}", a);
        println!("a = {:?}", b);
        println!("a = {:?}", c);
    }
}