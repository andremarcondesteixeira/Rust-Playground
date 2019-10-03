const MAX_POINTS: u32 = 1_000_000;

fn main() {
    println!("Max points: {}", MAX_POINTS);
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    let something = "abcde";
    println!("something: {}", something);
    let something = something.len();
    println!("something: {}", something);

    println!("i8 max value: {}", get_signed_integer_max_value(8) as i8);
    println!("i16 max value: {}", get_signed_integer_max_value(16) as i16);
    println!("i32 max value: {}", get_signed_integer_max_value(32) as i32);
    println!("i64 max value: {}", get_signed_integer_max_value(64) as i64);
    println!("i128 max value: {}", std::i128::MAX);

    let _a = 57u8;
}

fn get_signed_integer_max_value(bits: u8) -> i128 {
    let bits: u8 = bits - 1;
    let mut value: i128 = 2;
    let mut count: u8 = 1;

    loop {
        count = count + 1;
        value = value * 2;
        if count == bits {
            break;
        }
    }

    return value - 1;
}