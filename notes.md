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

## Control Flow

```rust
//Match may be a good alternative for lots of else if statements
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

// if in variable assignment
    let condition = true;
    let number = if condition { 5 } else { 6 };



    fn main() {
        let mut counter = 0;
        //variable assignment  of a loop. value is assigned to result on break
        let result = loop {
            counter += 1;

            if counter == 10 {
                break counter * 2;
            }
    };

    println!("The result is {}", result);

    //While Loops
    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    //for loops
    fn main() {
        let a = [10, 20, 30, 40, 50];

        for element in a.iter() {
            println!("the value is: {}", element);
        }
    }   

    fn main() {
        //typical way to do a while loop in rust rev reverses the range
        for number in (1..4).rev() {
            println!("{}!", number);
        }
        println!("LIFTOFF!!!");
    }
}

```

## Ownership

Rust language behaves differently differently based on wheter or not a value is on the stack or allocated onto the heap.

Stack - stores values, must have a known, fixed size. Pushing to stack fast

Heap - data in unkown size or size that may change should be on the heap. Searches for location, returns pointer. Allocating to heap slower.

String:: is allocated on heap, literals are not since by default they are immutable.

### Ownership Rules

- Each value in Rust has a variable called its owner.
- Can only have one owner at a time.
- When owner is out of scope, values are dropped.
- Reassignment is a "move" can no longer use previous variable for anything stored on the heap ex:  s1 = "hello"   s2 = s1     s1 is no longer valid
- clone() creates a deep copy

- Reassignment works normally for anything that has a fix size and is stored on the stack

```rust
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.
```

 - ownership is at the level of whatever scope it goes into 

** The ownership of a variable follows the same pattern every time: assigning a value to another variable moves it. When a variable that includes data on the heap goes out of scope, the value will be cleaned up by drop unless the data has been moved to be owned by another variable.

## References and Borrowing

references allow you to refer to a value without taking ownership - &

references(borrowed values) are not mutable by default      change(&mut s); allows s to be mutated inside the function change

At any given time, you can have either one mutable reference or any number of immutable references.
References must always be valid.

We can create slices using a range within brackets by specifying [starting_index..ending_index], where starting_index is the first position in the slice and ending_index is one more than the last position in the slice

## Structs


```rust
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    //let mut user1 = User {}
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };


    user1.email


    //struct update syntax, we can achieve the same effect with less code, as shown in Listing 5-7. The syntax .. specifies that the remaining fields not explicitly set should have the same value as the fields in the given instance.

     let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    fn main() {
        struct Color(i32, i32, i32);
        struct Point(i32, i32, i32);

        let black = Color(0, 0, 0);
        let origin = Point(0, 0, 0);
    }



```

## Methods
Defined within the context of a struct, enum or trait.

Rust has automatic referencing and dereferencing

## Enums 

enum values can only be one of its variants. 

```rust
enum IpAddrKind {
    V4,
    V6,
}

let four = IpAddrKind::V4;
let six = IpAddrKind::V6;

enum IpAddrKind {
        V4,
        V6,
    }

// struct using an enum
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// consise enum representation
enum IpAddr {
        V4(String),
        V6(String),
    }
    //consise
    let home = IpAddr::V4(String::from("127.0.0.1"));
    //consise
    let loopback = IpAddr::V6(String::from("::1"));

//verbose struct rep 
let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};
//verbose struct rep
let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
};
```

## Option Enum
 ```rust 
enum Option<T> {
    Some(T),
    None,
}


// Match with option enum used when need to handle Nulls
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
``` 

## Match Control Flow
```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

//Similar to a switch statement
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
    //Example of match statement with multiple expressions
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}


let some_u8_value = 0u8;
match some_u8_value {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    _ => (), // Do nothing with the rest of the values in the 0-255 spectrum of u8
}


//The if let syntax lets you combine if and let into a less verbose way to handle values that match one pattern while ignoring the rest.
if let Some(3) = some_u8_value {
        println!("three");
    }

```

## Managing Projects 

Packages: A Cargo feature that lets you build, test, and share crates
Crates: A tree of modules that produces a library or executable
Modules and use: Let you control the organization, scope, and privacy of paths
Paths: A way of naming an item, such as a struct, function, or module

use works like import path

as - creates an alias for an import
