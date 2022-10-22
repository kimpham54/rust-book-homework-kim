# rust-book-homework-kim

Working through the Rust book, I've always wanted to learn lower level programming. Some notes along the way, really just for me but maybe you'll appreciate them too.

goals:
- first goal (november 2022): learn ch. 1-11, 13, 19
- second goal (december 2022): learn 12, 14-18
- third goal  (january 2022): learn 19-21 end of book
- fourth goal (whenever!): walkthrough some examples in rust in action, or see if i can contribute to open source projects by now

updates:
- oct 21: started ch 1-2
- oct22: started ch 3

## THE RUST BOOK

### Chapter 1

your app code is in src/main.rs

`rustc main.rs`

#### cargo

`cargo new app_name`

`cargo build`

`cargo run`

`cargo check` if you don't want an executable and checks if it compiles

`cargo build --release` to compile it with optimizations

#### Macros, functions, methods in Rust
macro - writes code that writes other code, metaprogramming. println! expands to produce more code

Macros are executed at compile time. They generally expand into new pieces of code that the compiler will then need to further process.

functions are your everyday blocks of code which runs when it's called. most programs are designed to pass data and return data

methods are like functions but is used in the context of a struct type, first parameter being self. functions more loosey goosey 

lambda functions - small anonymous functions https://www.w3schools.com/python/python_lambda.asp

### Chapter 2

#### The standard library
prelude - set of items in the standard library generally in scope in every rust program https://doc.rust-lang.org/std/prelude/index.html

stdin - standard input allows you to handle user input

`let apple` - immutable

`let mut apple` - mutable variable

use std::io - standard library, in it is the input/output library. :: indicates scope, so io is part of (in the scope of) std

`std::io::stdin` - the stdin function from the io module which allows handling of user input

can write as `io::stdin()`, returns an instance of std::io::Stdin

#### References
if i do `.read_line(&mut apple)`, the method .read_line appends the standard input from the user and appends it into a string without overwriting its contents and passes that as an argument. the method can change the string's content. & indicates the argument is a reference, which lets multiple parts of your code access one piece of data without copying into memory many times. rust makes it safe and easy to use references. references are immutable by default, use &mut guess instead of &guess to make it mutable

#### enum, struct, matching

#### Result
Result - a value you get after read_line is executed, it is an enumeration (enum) type. the purpose of Result is to encode error-handling information

variants: `Ok` and `Err`. Ok - success. Err - fail with more info

An instance of result has an expect method that you can use. Err will cause the program to crash. ok will take the return value Ok is holding and return it for you to use. The value in this case is the number of bytes of the user's input.

Error handling to suppress warnings is the right way to write a program, rather than just use expect and let a program crash.

#### Crate
a crate is a collection of rust source code files

a binary crate is an executable

a library crate contains code intended to be used in other programs and cannot be executed on its own. add as a dependency in `Cargo.toml`.

`Cargo.lock` contains details of build history that is helpful for compiling in the future, it knows what versions worked

`cargo update`

`cargo doc --open` builds documentation based on methods and functions used

Rust has a strong, static type system. It also has type inference and can guess when something should be for example a string, didn't make you write the type when using String::new()

some number types
- i32: 32 bit number
- u32: unsigned 32 bit number (positive no - or + signs just +)
- i64: 64 bit

you can "shadow" variables, so if you created variable guess as string you can reuse guess. do this when you don't want to create two unique variables but want to conver a value from one type to another type

parse method on strings converts strings to another type. The parse method will only work on characters that can logically be converted into numbers and so can easily cause errors.  Because it might fail, the parse method returns a Result type like read_line. has an Ok and Err variant types. so no need to use expect which crashes on an error, instead try to handle the error

A match expression is made up of arms. An arm consists of a pattern to match against, and the code that should be run if the value given to match fits that arm’s pattern. Rust takes the value given to match and looks through each arm’s pattern in turn. Patterns and the match construct are powerful Rust features that let you express a variety of situations your code might encounter and make sure that you handle them all. interesting analogy. comparing arms, check each arm, one by one
The understore _ is a catchall value in rust, Err(_) catchall in the last arm if not an Ok

### Chapter 3

- keywords useful https://doc.rust-lang.org/book/appendix-01-keywords.html

#### variables, constants, shadowing shadow variables

```
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
```

This program first binds x to a value of 5. Then it creates a new variable x by repeating let x =, taking the original value and adding 1 so the value of x is then 6. Then, within an inner scope created with the curly brackets, the third let statement also shadows x and creates a new variable, multiplying the previous value by 2 to give x a value of 12. When that scope is over, the inner shadowing ends and x returns to being 6. When we run this program, it will output the following:

```
$ cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/variables`
The value of x in the inner scope is: 12
The value of x is: 6
```

python is similar
```
$ x = 4
$ x = x + 1
$ x
=> 5
```

by using let, the variable is still immutable, we just get to do some transofmrations on it. by using let we're effectively creating a new variable

so mut allows a variable's value change but it doesn't let you change the variable's type. so only value?

let allows you to reassign to anything. 'shadowing'

#### data types

rust is statically typed - rust must know the types of all variables at compile time

type annotation is the u32 in this line `let guess: u32 = "42".parse().expect("Not a number!");`

scalar type represents a single value: integer, floating point number, boolean, characters

number literals can be used: decimal, hex, octal, binary, byte u8 style

a good default is i32 says the book

integer overflow - your assigned integer type is outside the scope of some value

two's complement wrapping - not a good way to handle an error, happens on release mode, changes range. better way to handle is to use some rust methods: wrapping, checked, overflowing, saturating https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-overflow

float f32 vs f64 single vs double precision

operators https://doc.rust-lang.org/book/appendix-02-operators.html

char literals vs string literals. char is one character single quotes. string use double quotes

compound types: tuple and array

tuple can be mixed content, array has to be same type

- arrays for stack rather than on heap, and for fixed number of elements e.g. months in a year. vs vector which can grow

- rust doesn't allow invalid memory access for safety reasons, unlike some other languages it will stop running

## OTHER NOTES

## RUST IN ACTION

closure - anonymous or lambda

stderr error logs
stdoutput normal print logs

fields variable is annotated with the type `Vec<_>`. Vec is shorthand for _vector_, a collection type that can expand dynamically. The underscore (_) instructs Rust to infer the type of the elements.

`if let Ok(length) = fields[1].parse ::<f32>()` reads as “attempt to parse fields[1] as a 32-bit floating-point number and, if that is successful, then assign the number to the length variable.


Dangling pointers—Live references to data that has become invalid over the course of the program (see listing 1.3)

Data races—The inability to determine how a program will behave from run to run because external factors change (see listing 1.4)

Buffer overflow—An attempt to access the 12th element of an array with only 6 elements (see listing 1.5)

Iterator invalidation—An issue caused by something that is iterated over after being altered midway through (see listing 1.6)


