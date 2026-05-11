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

// conversion data type

#[test]
fn number_conversion() {
    let a : i8 = 10;
    println!("The value of a is: {}", a);

    let b : i16 = a as i16;
    println!("The value of b is: {}", b);

    let c : i32 = a as i32;
    println!("The value of c is: {}", c);

    let d : i64 = 1000000000000;

    let e : i8 = d as i8;
    println!("The value of e is: {}", e);
}

// Numeric Operations

#[test]
fn numeric_operations() {
    let a = 10;
    let b = 10;

    let c = a * b;
    let d = a / b;
    let e = a + b;
    println!("The value of c is: {}", c);
    println!("The value of d is: {}", d);
    println!("The value of e is: {}", e);
}

// Augmented Assignment

#[test]
fn augmented_assignment() {
    let mut a = 10;
    a = a + 5; // basic assignment
    a += 5;
    println!("The value of a is: {}", a);
}

// Boolean Type

#[test]
fn boolean() {
    let a = true;
    let b = false;
    println!("The value of a is: {}", a);
    println!("The value of b is: {}", b);
}

// Comparison Operators

#[test]
fn comparison_operators() {
    let a = 20;
    let b = 20;

    let c = a == b;
    println!("The value of c is: {}", c);
}

// Boolean Operators

#[test]
fn boolean_operators() {
    let a = true;
    let b = false;

    let c = a && b;
    let d = a || b;
    let e = !a;

    println!("The value of c is: {}", c);
    println!("The value of d is: {}", d);
    println!("The value of e is: {}", e);
}

// Character Type

#[test]
fn char() {
    let a = 'A';
    let b = 'B';
    println!("The value of a is: {}", a);
    println!("The value of b is: {}", b);
}

//  Tuple Type

#[test]
fn tuple() {
    let a = (1, "Marchel", 'a');
    println!("The value of a is: {:?}", a);
    println!("The value of a.0 is: {}", a.0); // Cara mengambil nilai dari tuple dengan menggunakan index
    println!("The value of a.1 is: {}", a.1);
    println!("The value of a.2 is: {}", a.2);  
}

// Destructuring Tuple

#[test]
fn destructuring_tuple() {
    let a = (1, "Marchel", 'a');
    let (x, y, _) = a; // Cara mengambil nilai dari tuple dengan menggunakan destructuring
    // Gunakan _ jika tidak ingin mengambil spesifik nilai dari tuple
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

// Mutable Tuple

#[test]
fn mutable_tuple() {
    let mut a = (1, "Marchel", 'a');
    println!("The value of x is: {:?}", a);

    a.0 = 2; // Cara mengubah nilai dari tuple dengan menggunakan index
    a.1 = "Marchel M";
    a.2 = 'b';
    println!("The value of x is: {:?}", a);
}

// Empty Tuple or Unit Type

fn unit() {
    println!("Hello");
}

#[test]
fn test_unit() {
    let a = unit(); 
    println!("The value of a is: {:?}", a);
}
