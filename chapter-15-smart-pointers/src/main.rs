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

    let x2 = 5;
    let y2 = Box::new(x);
    assert_eq!(5, x2);
    assert_eq!(5, *y2);
}

enum List<T> {
    Cons(T, Box<List<T>>),
    Nil
}