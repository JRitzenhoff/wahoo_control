# Notes for the programming language Rust
Helpful beginner book for the language: https://doc.rust-lang.org/book

Helpful example filled book for the language: https://doc.rust-lang.org/rust-by-example

## Entrypoint
The configuration data for `cargo` (the package manager/runner) is saved in the "Cargo.toml" file


## Project creation

`cargo new <project name>`

## Library/Module creation

`cargo new --lib <libarary name>`


## Cargo.toml

Contains:
* Information about the actual package name, version, edition
* Information about the dependencies of this project
* The name of the executable
* The entrypoint for the executable

## Slicing

* `[0..5]` is equivalent to `[0,5)` (inclusive on the lower and exclusive on the upper... `[0, 1, 2, 3, 4]`)
* `[0..=5]` is equivalent to `[0,5]` (inclusive on the lower and inclusive on the upper... `[0, 1, 2, 3, 4, 5]`)

## Casting

Explicit casting can be achieved using the `as` keyword

```rust
let std_32_int: i32 = 10024;
let std_8_int = std_32_int as i8; 
```

```rust
impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}
```

## Walrus Operator Equivalent

`if let _ = _ {}` enables the assignment of a variable in the "if" block


## Converions

Can use the `From<>` and `Into<>` traits. This will be helpful when doing JSON conversions.

```rust
use std::convert::From;

struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}
```

## Nested Loop Breaks

```rust
#![allow(unreachable_code)]

fn main() {
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // This would break only the inner loop
            //break;

            // This breaks the outer loop
            break 'outer;
        }
        println!("This point will never be reached");
    }
    println!("Exited the outer loop");
}
```

NOTE: One can also assign the result from a loop to variable

```rust
let mut y = 9;
let x = loop { 
    if y > 10 {
        // x will be assigned the value of this here
        break y;
    }

    y += 2;
}
```

## Iteration

There exist three forms of iterator calls: `into_iter`, `iter` and `iter_mut`
* `into_iter` consumes the real values
* `iter` borrows the real values
* `iter_mut` allows for the manipulation of the real values

## impl

This can be used to define methods related to a struct AND as a pseudo "implements" keyword in return statements

## Struct destructuring ('update' syntax)

It is possible to define a new struct, only declare the desired field modifications, and provide an existing struct of the same type to populate the remaining fields.

```rust
struct Person {
    name: &str,
    age: u8,
    height_in_cm: f32
}

let sara = Struct { name: "Sara", age: 43, height_in_cm: 150 };
let twin = Struct { name: "Molly", ..sara };
// the "twin" will also have an 'age' and 'height_in_cm' copied from the sara instance 
```

## Lifetime parameters

The `'_` "type" descriptor refers the lifetime of a variable.

Some variables have pointers to objects. One needs to make sure that the pointer can never reference something unintended.
Otherwise this will result in mis-used memory and can crash the program unexpectedly.

Therefore, it is possible to describe that variables need to have a certain lifetime. It's possible to therefore declare that a wrapper is going to exist at least as long as the object it wraps. This allows you to use references without fear of ever pointing to something different than expected.

## Referencing and dereferencing

Don't have a fantastic concept of this just yet.

* Reference Operator: `&` 
* Dereference Operator: `*`

I believe referencing is basically turning an object into a pointer so that it can be modified by other variables.

Dereferencing is converting that pointer to a struct again (so a dereference requires that the struct implements "Copy") to memcopy it's original binary to the new variable. 

More explained here: https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html