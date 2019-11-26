fn main() {
    let rectangle = Rectangle {
        width: 50,
        height: 30,
    };

    let rectangle2 = Rectangle::square(20);

    let can_hold = rectangle.can_hold(&rectangle2);

    println!(
        "The area of the rectangle1 is {} square pixels.",
        rectangle.area()
    );
    println!(
        "The area of the rectangle2 is {} square pixels.",
        rectangle2.area()
    );
    println!("Rectangle1 can hold rectangle2: {}", can_hold);
    println!("rectangle1 is {:#?}", rectangle);
    println!("rectangle2 is {:#?}", rectangle2);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, holdee: &Rectangle) -> bool {
        self.width > holdee.width && self.height > holdee.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
