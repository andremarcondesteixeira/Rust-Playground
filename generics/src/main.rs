fn main() {
    //experiment1();
    let point = Point { x: 'a', y: 'b' };
    point.foo();
    point.foo2();
    point.foo4();

    let _pointb = Point { x: 1, y: 2 };
    let _ok: Result<char, char> = Ok('a');
    let _brutus: HeroOrVilain<(), bool> = HeroOrVilain::Vilain(true);

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

// fn experiment1() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let result = largest(&number_list);
//     println!("The largest number is {}", result);

//     let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

//     let result = largest(&number_list);
//     println!("The largest number is {}", result);
// }

// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

struct Point<T, Z> {
    x: T,
    y: Z,
}

impl<T, Z> Point<T, Z> {
    fn foo(&self) {
        println!("Generic types are different");
    }

    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

impl<T> Point<T, T> {
    fn foo2(&self) {
        println!("Generic types are the same");
    }
}

impl<T> SameTypes for Point<T, T> {
    fn foo4(&self) {
        println!("I also have a trait for same types! :D");
    }
}

impl Point<char, u32> {
    fn _foo3(&self) {
        println!("Generic types are different");
    }
}

trait SameTypes {
    fn foo4(&self);
}

enum HeroOrVilain<H, V> {
    _Hero(H),
    Vilain(V),
}
