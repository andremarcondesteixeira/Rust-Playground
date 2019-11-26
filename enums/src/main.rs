fn main() {
    let quit_message = Message::Quit;
    quit_message.call();

    let move_message = Message::Move {
        x: 24,
        y: 560,
    };
    move_message.call();

    let write_message = Message::Write(String::from("foo"));
    write_message.call();

    let change_color_message = Message::ChangeColor(32, 350, 200);
    change_color_message.call();

    let coin = Coin::Penny;
    println!("A penny is worth {:?} cent", value_in_cents(coin));

    let coin = Coin::Nickel;
    println!("A nickel is worth {:?} cents", value_in_cents(coin));

    let coin = Coin::Dime;
    println!("A dime is worth {:?} cents", value_in_cents(coin));

    let coin = Coin::Quarter(UsState::Alabama);
    println!("A quarter is worth {:?} cents", value_in_cents(coin));

    let coin = Coin::Quarter(UsState::Alaska);
    println!("A quarter is worth {:?} cents", value_in_cents(coin));

    let five = Some(5);
    println!("five is {:?}", five);
    let six = plus_one(five);
    println!("six is {:?}", six);
    let none = plus_one(None);
    println!("none is {:?}", none);
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("{:#?}", self);
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!!!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
        //Coin::Quarter(_) => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(v) => Some(v + 1),
    }
}
