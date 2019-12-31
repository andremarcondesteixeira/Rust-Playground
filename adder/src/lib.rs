#[cfg(test)]
mod tests {
    #[test]
    fn test_something() {
        assert_eq!(2 + 2, 4);
    }

    // #[test]
    // fn fail() {
    //     panic!("failed");
    // }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };

        assert!(larger._can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };

        assert!(!smaller._can_hold(&larger));
    }

    #[test]
    fn add_two_works() {
        assert_eq!(4, _add_two(2), "Oh, snap!");
    }

    #[test]
    #[should_panic(expected="This message helps filter the panic cause")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    use super::*;
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn _can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn _add_two(n: i32) -> i32 {
    n + 2
}

pub struct Guess {
    _value: i32,
}

impl Guess {
    pub fn new(_value: i32) -> Guess {
        if _value < 1 || _value > 100 {
            panic!("This message helps filter the panic cause");
        }

        Guess {
            _value
        }
    }
}
