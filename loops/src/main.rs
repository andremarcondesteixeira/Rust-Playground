fn main() {
    // numero primo: numero divisivel apenas por um e por ele mesmo.
    let mut divider: u32;
    let mut is_prime_number: bool;

    for number in 2..101 {
        divider = number - 1;

        is_prime_number = loop {
            if divider == 1 {
                break true;
            }

            if number % divider == 0 {
                break false;
            }

            divider -= 1;
        };

        if is_prime_number == true {
            println!("{}", number);
        }
    }
}