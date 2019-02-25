fn main() {
    // var 1 immutable
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // var 2 shadowing
    let y = 5;

    let y = y + 1;

    let y = y * 2;

    println!("The value of y is: {}", y);

    // var 3 change type shadow
    let spaces = "   ";
    let spaces = spaces.len();

    // data type 1 specify type
    let guess: u32 = "42".parse().expect("Not a number!");

    // data type 2 floating point
    let a = 2.0; // f64

    let b: f32 = 3.0; // f32

    //data type 3 bool
    let t = true;

    let f: bool = false; // with explicit type annotation

    // data type 4 char
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    //data type 5 tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (i, j, k) = tup;

    println!("The value of j is: {}", j);
}
