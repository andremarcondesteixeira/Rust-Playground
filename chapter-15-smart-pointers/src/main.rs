use std::ops::Deref;

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    let _cons = List::Cons::<i32>(1,
        Box::new(List::Cons(2,
            Box::new(List::Cons(3,
                Box::new(List::Cons(4,
                    Box::new(List::Nil))))))));

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
}

enum List<T> {
    Cons(T, Box<List<T>>),
    Nil
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}