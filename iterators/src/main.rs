struct Counter {
    count: u32,
}

impl Counter {
    fn _new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {}

#[test]
fn calling_next_directly() {
    let mut counter = Counter::_new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

#[test]
fn testing_the_map_function() {
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
}

#[test]
fn testing_the_filter_function() {
    #[derive(PartialEq, Debug)]
    struct Shoe {
        size: u8,
        style: String,
    }
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 12,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];
    let my_size = 10_u8;
    let filtered: Vec<Shoe> = shoes.into_iter().filter(|x| x.size == my_size).collect();
    assert_eq!(
        filtered,
        vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            }
        ]
    );
}

#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::_new()
        .zip(Counter::_new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(18, sum);
}
