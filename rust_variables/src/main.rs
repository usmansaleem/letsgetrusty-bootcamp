fn main() {
    const MAX_PLAYERS: u8 = 10;
    // creation
    let a: i16 = 5;

    // mutability
    let mut b = 5;
    b = 20;

    // shadowing
    let c = 10;
    let c = 30;

    println!("c is {c}");

    // scope
    let d = 30;
    {
        let e = 60;
        println!("e in {e}");
    }

    println!("d is {d}");
}

fn my_func(x: i32) {
    println!("my_function called with {}", x);
}
