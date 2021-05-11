fn main() {
    let first_name = String::from("André");
    let last_name = String::from("Teixeira");
    let complete_name = |first_name: String| -> String {
        let mut complete_name = first_name;
        complete_name.push_str(" ");
        complete_name.push_str(&last_name);
        complete_name
    };
    println!("{}", run(complete_name, first_name));

    struct My {
        x: String,
    }

    let my_struct = My {
        x: String::from("x"),
    };

    let callback = || my_struct.x;

    println!("{}", callback());
}

fn run<Closure, ClosureParameterType, ClosureReturnType>(
    callback: Closure,
    callback_parameter: ClosureParameterType,
) -> ClosureReturnType
where
    Closure: Fn(ClosureParameterType) -> ClosureReturnType,
{
    callback(callback_parameter)
}
