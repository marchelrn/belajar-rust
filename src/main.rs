fn main() {
    hello_world();
}

fn hello_world() {
    println!("Hello, world!");
}

#[test]
fn test_hello_world() {
    hello_world();
}

#[test]
fn test_variable() {
    let name = "Marchel";
    println!("Hello, {}!", name);
}

#[test]
fn test_mutable() {
    let mut m = "Marchel M";
    println!("The value of x is: {}", m);
    m = "M Marchel";
    println!("The value of y is: {}", m);
}

#[test]
fn shadowing() {
    let x = 5;
    println!("The value of x is: {}", x);

    let x = "Marchel";
    println!("The value of x is: {}", x);
}

#[test]
fn comment() {
    // line ini tidak akan di compile
    println!("line ini akan di compile");

    /*
    Dua line ini
    Tidak akan di compile
     */
    println!("Dua line ini");
    print!("Akan di compile");
}

// Explicit Type
#[test]
fn explicit() {
    let x: i32 = 5;
    println!("The value of x is: {}", x);
}

// Data Type

/*
 - Scalar Type
    Misal : Integer, Float, Boolean, Char
 - Compound Type
    Misal : Tuple, Array
*/

#[test]
fn number() {
    let x: i8 = 10;
    println!("The value of x is: {}", x);

    let y: f32 = 10.0;
    println!("The value of y is: {}", y);
}



