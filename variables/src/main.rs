fn main() {
    println!("i8 max value: {}, {}", get_signed_integer_max_value(8u8) as i8, std::i8::MAX);
    println!("i16 max value: {}, {}", get_signed_integer_max_value(16u8) as i16, std::i16::MAX);
    println!("i32 max value: {}, {}", get_signed_integer_max_value(32u8) as i32, std::i32::MAX);
    println!("i64 max value: {}, {}", get_signed_integer_max_value(64u8) as i64, std::i64::MAX);
    println!("i128 max value: {}", std::i128::MAX); // can't calculate this value using the get_signed_integer_max_value function because the max value would be overflown by 1

    println!("i8 min value: {}, {}", get_signed_integer_min_value(8u8) as i8, std::i8::MIN);
    println!("i16 min value: {}, {}", get_signed_integer_min_value(16u8) as i16, std::i16::MIN);
    println!("i32 min value: {}, {}", get_signed_integer_min_value(32u8) as i32, std::i32::MIN);
    println!("i64 min value: {}, {}", get_signed_integer_min_value(64u8) as i64, std::i64::MIN);
    println!("i128 min value: {}, {}", get_signed_integer_min_value(128u8) as i128, std::i128::MIN);

    println!("u8 max value: {}, {}", get_unsigned_integer_max_value(8u8) as u8, std::u8::MAX);
    println!("u16 max value: {}, {}", get_unsigned_integer_max_value(16u8) as u16, std::u16::MAX);
    println!("u32 max value: {}, {}", get_unsigned_integer_max_value(32u8) as u32, std::u32::MAX);
    println!("u64 max value: {}, {}", get_unsigned_integer_max_value(64u8) as u64, std::u64::MAX);
    println!("u128 max value: {}", std::u128::MAX); // can't calculate this value using the get_unsigned_integer_max_value function because the max value would be overflown by 1
}

/* Each signed variant can store numbers from -(2^(n-1)) to 2^(n-1)-1 inclusive,
where n is the number of bits that variant uses */
fn get_signed_integer_max_value(bits: u8) -> i128 {
    let bits: u8 = bits - 1u8;
    let mut value: i128 = 2;
    let mut count: u8 = 1;

    loop {
        count = count + 1u8;
        value = value * 2i128;
        if count == bits {
            break;
        }
    }

    return value - 1i128;
}

/* Each signed variant can store numbers from -(2^(n-1)) to 2^(n-1)-1 inclusive,
where n is the number of bits that variant uses */
fn get_signed_integer_min_value(bits: u8) -> i128 {
    // When 1 is added to a integer type's maximum value, the result is the type's minimum value,
    // so, could just do this if the amount of bits is less than 128 (because otherwise the program
    // would crash at runtime)
    // return get_signed_integer_max_value(bits) + 1i128;

    let bits: u8 = bits - 1u8;
    let mut value: i128 = -2;
    let mut count: u8 = 1;

    loop {
        count = count + 1u8;
        value = value * 2i128;
        if count == bits {
            break;
        }
    }

    return value;
}

/* Unsigned variants can store numbers from 0 to (2^n)-1 */
fn get_unsigned_integer_max_value(bits: u8) -> u128 {
    let mut value: u128 = 2;
    let mut count: u8 = 1;

    loop {
        count = count + 1;
        value = value * 2;
        if count == bits {
            break;
        }
    }

    return value - 1u128;
}