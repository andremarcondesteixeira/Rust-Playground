fn main() {
    let brand = String::from("VFuse");
    let car = Car {
        brand,
        model: String::from("Prototype 152"),
        engine: Engine {
            engine_type: String::from("V16"),
            horsepowers: 2000,
        },
    };
    
    print_car(&car);

    let car2 = Car {
        model: String::from("Prototype 153"),
        engine: Engine {
            horsepowers: 2250,
            ..car.engine
        },
        ..car
    };

    print_car(&car2);

    fn print_car(car: &Car) {
        println!("This is my car:");
        println!("Brand: {}", car.brand);
        println!("Model: {}", car.model);
        println!("Engine:");
        println!("  Type: {}", car.engine.engine_type);
        println!("  Power (HP): {}", car.engine.horsepowers);
    }

    struct Car {
        brand: String,
        model: String,
        engine: Engine,
    }

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("Black => R: {}, G: {}, B: {}", black.0, black.1, black.2);
    println!("Orgin => X: {}, Y: {}, Z: {}", origin.0, origin.1, origin.2);
}

struct Engine {
    engine_type: String,
    horsepowers: u16,
}
