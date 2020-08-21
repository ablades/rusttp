fn main() {
    
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is {}", x);

    let x = 2.0; // f64

    let yy: f32 = 3.0; // f32


    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    let t = true;

    // explicit type annotation
    let f: bool = false;

    //char - unicode scalar value
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    //tuple type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    //multiple assignment - destructuring
    let (x, y, z) = tup;

    //assignment of value by tuple index
    let five_hundred = tup.0;

    //arrays are fixed in size - good for constants (months in a year)
    let a = [1, 2, 3, 4, 5];

    //array that specifys type and size
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    //array that specifys value and allocations of that value [3, 3, 3, 3, 3]
    let a = [3; 5];

    a[0] // get first element
    //vectors are scalable in size

}

//function signature with parameter name: type
fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}