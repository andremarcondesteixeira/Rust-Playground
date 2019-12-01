pub fn run_my_experiments() {
    let mut numbers = create_a_vector_using_new();
    let strings = create_a_vector_using_the_vec_macro();
    
    for number in 1..10 {
        numbers.push(number * 2);
    }

    for n in numbers {
        println!("number is {}", n);
    }

    for s in strings {
        println!("{}", s);
    }
}

fn create_a_vector_using_new() -> Vec<i32> {
    let v: Vec<i32> = Vec::new();
    v
}

fn create_a_vector_using_the_vec_macro() -> Vec<String> {
    vec![
        String::from("foo"),
        String::from("bar"),
        String::from("baz"),
        String::from("qux"),
    ]
}