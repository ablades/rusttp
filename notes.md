# Rust Notes

## Keywords
> - as - perform primitive casting, disambiguate the specific trait containing an item, or rename items in use and extern crate statements
> -  async - return a Future instead of blocking the current thread
> - await - suspend execution until the result of a Future is ready
> - break - exit a loop immediately
> - const - define constant items or constant raw pointers
> - continue - continue to the next loop iteration
> - crate - link an external crate or a macro variable representing the crate in which the macro is defined
> - dyn - dynamic dispatch to a trait object
> - else - fallback for if and if let control flow constructs
> - enum - define an enumeration
> - extern - link an external crate, function, or variable
> - false - Boolean false literal
> - fn - define a function or the function pointer type
> - for - loop over items from an iterator, implement a trait, or specify a higher-ranked lifetime
> - if - branch based on the result of a conditional expression
> - impl - implement inherent or trait functionality
> - in - part of for loop syntax
> - let - bind a variable
> - loop - loop unconditionally
> - match - match a value to patterns
> - mod - define a module
> - move - make a closure take ownership of all its captures
> - mut - denote mutability in references, raw pointers, or pattern bindings
> - pub - denote public visibility in struct fields, impl blocks, or modules
> - ref - bind by reference
> - return - return from function
> - Self - a type alias for the type we are defining or implementing
> - self - method subject or current module
> - static - global variable or lifetime lasting the entire program execution
> - struct - define a structure
> - super - parent module of the current module
> - trait - define a trait
> - true - Boolean true literal
> - type - define a type alias or associated type
> - union - define a union and is only a keyword when used in a union declaration
> - unsafe - denote unsafe code, functions, traits, or implementations
> - use - bring symbols into scope
> - where - denote clauses that constrain a type
> - while - loop conditionally based on the result of an expression

## Variables and Mutability
Variables are immutable by default.

```rust
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
```

## Functions
**Statements** are instructions that perform some action and do not return a value.
- function definitions are statements
- setting a variable is a statement
- statements end in a semicolon

**Expressions** evaluate to a resulting value.
- expressions do not end in a semicolon!

```rust
//function signature with parameter name: type
fn another_function(x: i32) {
    println!("The value of x is: {}", x);

    //6 is an expression that evaluates to 6
    let y = 6;
}

fn main() {
    let x = 5;

    //inner is an expression that assigns y to be x + 1
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}

//return type is i32
//functions automatically return the last expression implicitly
fn five() -> i32 {
    5 //implicit return
}

let x = five();

fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

```