use std::cell::RefCell;
use std::rc::Rc;
use crate::reference_cycle::List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(&item),
            Nil => None,
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        println!("This list is going away now")
    }
}

#[cfg(test)]
mod test {
    use std::borrow::BorrowMut;
    use super::*;

    #[test]
    fn create_cycle() {
        {
            let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
            println!("a initial reference count: {}", Rc::strong_count(&a));
            println!("a next item: {:?}", a.tail());
        }
        let b = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
        println!("b initial reference count: {}", Rc::strong_count(&b));
        println!("b next item: {:?}", b.tail());

        let c = Rc::new(Cons(10, RefCell::new(Rc::clone(&b))));
        println!("b reference count after c creation: {}", Rc::strong_count(&b));
        println!("c initial reference count: {}", Rc::strong_count(&c));
        println!("c next item: {:?}", c.tail());

        if let Some(link) = b.tail() {
            *link.borrow_mut() = Rc::clone(&c);
        }

        println!("c reference count after changing b to point to c: {}", Rc::strong_count(&c));
        println!("b reference count after changing b to point to c: {}", Rc::strong_count(&b));

        // Printing a cycle is fun but causes a stack overflow
        // println!("b next item: {:?}", b.tail());

        // Running this test shows that b and c head nodes are never dropped - memory leak!

    }
}