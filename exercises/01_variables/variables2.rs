fn main() {
    // TODO: Change the line below to fix the compiler error.
    let x: u8 = 10;
    is10(x);

    let x = 8;
    is10(x)
}

fn is10(x: i8) {
    if x == 10 {
        println!("x is ten!");
    } else {
        println!("x is not ten!");
    }
}
