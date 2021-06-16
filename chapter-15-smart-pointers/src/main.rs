use std::{fmt::Display, ops::Deref, rc::Rc};

mod interior_mutability;
mod tree;

fn main() {
    use List::{Cons, Nil};

    let b = Box::new(5);
    println!("b = {}", b);

    let _cons = Rc::new(
        Cons(1, Rc::new(
            Cons(2, Rc::new(
                Cons(3, Rc::new(
                    Cons(4, Rc::new(
                        Nil)))))))));

    let _cons2 = Cons(1, Rc::clone(&_cons));
    let _cons2 = Cons(1, Rc::clone(&_cons));

    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    tree::run();
}

enum List<T> {
    Cons(T, Rc<List<T>>),
    Nil
}

#[derive(Debug)]
struct MyBox<T: Display>(T);

impl<T: Display> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T: Display> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T: Display> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("{} was dropped", self.0);
    }
}
