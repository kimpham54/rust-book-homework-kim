# rust-book-homework-kim

Working through the Rust book, I've always wanted to learn lower level programming. Some notes along the way, really just for me but maybe you'll appreciate them too.

goals:
- first goal (november 2022): learn ch. 1-11, 13, 19
- second goal (december 2022): learn 12, 14-18
- third goal  (january 2022): learn 19-21 end of book
- fourth goal (whenever!): walkthrough some examples in rust in action, or see if i can contribute to open source projects by now

updates:
- oct22: finished ch 1,2, started ch 3
- oct23: finished ch 3, started ch 4
- oct27: finished ch 4


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
macro - code shortcut, code that writes other code, aka metaprogramming. println! expands to produce more code

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

A match expression is made up of arms. An arm consists of a pattern to match against, and the code that should be run if the value given to match fits that arm’s pattern. Rust takes the value given to match and looks through each arm’s pattern in turn. 

Patterns and the match construct are rust features to express a variety of situations your code might encounter and make sure that you handle them all. interesting analogy, like some octopus multitasker matchmaker. comparing arms, check each arm, one by one

The understore _ is a catchall value in rust, Err(_) catchall in the last arm if not an Ok

### Chapter 3 - Control Flow

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
inner scope let statement shadows x and creates new variable value of x is 12. however when scope is over, inner shadowing ends and x returns to being 6.

output:

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

- question: so mut allows a variable's value change but it doesn't let you change the variable's type. so only value?

let allows you to reassign to anything. 'shadowing'

shadowing is different from mutability - shadow is essentially new

constant is different from immutable variables (const can be global)

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

#### functions
- rust is an expression based language. it has statements and expressions. Statements are instructions that perform some action and do not return a value. Expressions evaluate to a resulting value

let is a statement, function definition is a statement

`let x = (let y = 6);` would cause an error

The let y = 6 statement does not return a value, so there isn’t anything for x to bind to. This is different from what happens in other languages, such as C and Ruby, where the assignment returns the value of the assignment. In those languages, you can write x = y = 6 and have both x and y have the value 6; that is not the case in Rust.

Calling a function is an expression. Calling a macro is an expression. A new scope block created with curly brackets is an expression

Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value

order of function declarations don't matter

mismatched types -> functions should return values. don't use semicolon or else it'd be a statement at end of function

#### control flow

blocks of code associated with the conditions in if expressions are also sometimes called arms

```
fn main() {
    let number = 3;

    if number {
        println!("number was three");
    }
}
```
The error indicates that Rust expected a bool but got an integer. Unlike languages such as Ruby and JavaScript, Rust will not automatically try to convert non-Boolean types to a Boolean.

Using too many else if expressions can clutter your code, so if you have more than one, you might want to refactor your code. Chapter 6 describes a powerful Rust branching construct called match for these cases.

`let number = if condition { 5 } else { "six" };` won't work cause if and else arms have to be the same value type because variables must have a single type and rust needs to know at compile time what type it will be: 'Rust wouldn’t be able to do that if the type of number was only determined at runtime; the compiler would be more complex and would make fewer guarantees about the code if it had to keep track of multiple hypothetical types for any variable.'

know: 
- if, else, else if
- while - loop if else break
- loop break continue, loops can be labelled
- for - nice because consistent and concise

### Chapter 4 - Ownership

- this chapter is about memory allocation
- languages like java do garbage collection, others make you explicitly allocate and free the memory. rust uses ownership
- stack (like the algorithm) - last in, first out like plate stacks. you push and pop. data must have a known fixed size
- heap - finds an empty spot, puts it there, gets a pointer. pointers can be put on a stack, when you want actual data you follow the pointer. restaurant seat at a table analogy
- stack is generally more efficient (no pointers, no searching)
- ownership main function is to manage heap data. Keeping track of what parts of code are using what data on the heap, minimizing the amount of duplicate data on the heap, and cleaning up unused data on the heap so you don’t run out of space are all problems that ownership addresses.


> Ownership rules:
> Each value in Rust has an owner.
> There can only be one owner at a time.
> When the owner goes out of scope, the value will be dropped.

- String data is managed/allocated on the heap
- question answered: types with known size are allocated on the stack? like integers? YES

- string literals are immutable, put into the binary at compile time. String types are mutable and memory is allocated on the heap for those boys

- when a variable is out of scope the function drop is called in rust to deallocate memory, other languages make you do this manually

these two are not the same. because of memory allocation. integers go on the stack, strings go in the heap. you can't just call s1. if you called s2 in the println! that would work. due to ownership and borrowing. if you used the clone method you could call s1. has to do with memory allocation and pointers

```
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}, world!", s1);
```

```
    let s1 = 6;
    let s2 = s1;
    println!("{}, world!", s1);
```

- question: wondering how the pointers work when a variable gets reassigned.

```
let x = 6;
let y = x;
let x = 5;
would y equal to 6 or 5? the answer is 6. y will never auto update if x changes so the pointer diagram is slightly wrong in fig 4-3?
```

- **clone** method
- **Copy trait** - use on stack types, won't be invalidated
- **Drop trait** - invalidated or cleaned from memory when out of scope
- **move** vs drop

#### References and borrowing
- instead of using the same variable and calling it around and using tuples to achieve this, the clean way to do this is to use references
- **reference (&)**: like a pointer to access data that is owned by some other variable. unlike a pointer a ref is guaranteed to point to a valid value while it exists
- that way you don't have to create a new variable to use it again/no need to trtakeansfer ownership. you just borrow.
- **borrowing** - the action of creating a reference
- references are immutable by default, We’re not allowed to modify something we have a reference to, can do with mutable references - &mut varname. original variable needs to be mutable too
- borrowing is strict, you can't have multiple mutable references until it's done being used the first time to control for mutation in a controlled fashion - i'm guessing this is for safety reasons - ah it prevents data races at compile time

A **data race** is similar to a race condition and happens when these three behaviors occur:

- Two or more pointers access the same data at the same time.
- At least one of the pointers is being used to write to the data.
- There’s no mechanism being used to synchronize access to the data.

- curly brackets new scope to ref the same variable but rust will not allow simultaneous references

- hmm the code is really designed with safety in mind - so if you're ever trying to debug something just remember that and think what's logically the safest, that could be why it's not working

- variable gets released to be borrowed again once it is used (aka when scopes don't overlap) = Non Lexical Lifetimes

- **dangling pointer** pointer that references a location in memory that may have been given to someone else, frees memory but preserves a pointer to that memory. rust will not let compile if data goes out of scope before its reference does

#### The Slice Type
- slice like in python but the definition is worded differently! it's a reference so does not have ownership
- enumerate, iterate, byte literal, as_bytes
- function arrow returning values not tied to state, e.g. fn second_word(s: &String) -> (usize, usize) {
- string slices &str, you don't have ownership of data in this case. this was a really nice explanation: https://blog.thoughtram.io/string-vs-str-in-rust/. this helped too https://www.youtube.com/watch?v=8M0QfLUDaaA or https://www.openmymind.net/Rust-Strings/

types of borrowing:
- mutable reference looks like -> `&mut value` -> mutable borrow
- immutable reference looks like -> `&value` even if the original declaration is mut -> read only borrow
- are all subsequent mentions of a variable a reference, aka borrowed?


> Rule 1: If you have only immutable references, you can have as many as you want. 1 is fine, 3 is fine, 1000 is fine. No problem.
> Rule 2: If you have a mutable reference, you can only have one at a time. Also, you can't have an immutable reference and a mutable reference together.

- ok String and str is pretty hard to understand. https://blog.thoughtram.io/string-vs-str-in-rust/. here goes:
- String std::String::String objects have its buffer, capacity, and length
- String slices str reference a range of text owned by someone else (that is tied to some underlying data), or are strong literals e.g. let mut dog = "Merle". they don't have a capacity. are stored in read only memory

the concepts of ownership, borrowing, slices is important to write memory safe programs

https://dhghomon.github.io/easy_rust/Chapter_11.html

i had to read over this chapter again and again. finally posting on stack overflow, reddit, and discord. it's hard to wrap your head around, it's something you take for granted with other languages. it's going to take time for this to feel intuitive

### Chapter 5 - Structs

- structs like in OOP are custom types for your domain. they are good for organization, have fields like attributes. you can have instances
- struct uses = for assignment. if you use the values from user1 in user 2 it moves data if its a String type, for boolean/integer it's a Copy. 
- structs aree its own type, even if they have similar values within their types
- custom types in your API ensures type safety by compiler making sure your functions only get values of the type each function expects

```
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

```

> The tuple without any values has a special name, unit. This value and its corresponding type are both written () and represent an empty value or an empty return type. Expressions implicitly return the unit value if they don’t return any other value.

> almost everything is an expression in rust. evaluates to something

- strcuts are useful when you want to add more meaning e.g. in writing your functions

more Rust traits https://doc.rust-lang.org/book/appendix-03-derivable-traits.html

- Debug (from derive)

- an alternative to println! is dbg! you can wrap around an expression, or object

can use:

```
#[derive(Debug)]
#![allow(dead_code)]
```

i opened a ticket: https://github.com/rust-lang/book/issues/3409

#### Methods
- functions in the context of a struct, enum, or trait object
- first parameter is self
- use the `impl` to start a block associated with your struct type
- use &self if you just want to borrow the Self instance, as they can take ownership of self, borrow immutably (using &self) or borrow mutable (&mut self)
- If we wanted to change the instance that we’ve called the method on as part of what the method does, we’d use &mut self as the first parameter. 

if you use &mut self in this way:

new dog1 =  Dog {
    name: 'merle'
    colour: 'read',
}

and did 

&mut self and change the name and did it for a new dog2, does it change dog1 and add to dog2

from book, answered my reddit question

```
Rust doesn’t have an equivalent to the -> operator; instead, Rust has a feature called automatic referencing and dereferencing. Calling methods is one of the few places in Rust that has this behavior.

Here’s how it works: when you call a method with object.something(), Rust automatically adds in &, &mut, or * so object matches the signature of the method. In other words, the following are the same:

p1.distance(&p2);
(&p1).distance(&p2);
The first one looks much cleaner. This automatic referencing behavior works because methods have a clear receiver—the type of self. Given the receiver and name of a method, Rust can figure out definitively whether the method is reading (&self), mutating (&mut self), or consuming (self). The fact that Rust makes borrowing implicit for method receivers is a big part of making ownership ergonomic in practice.
```

- all functions within impl block are associated functions
- associated functions are methods (self as first parameter) or can also be not methods (e.g. constructors that return a new instance of the struct)
- use :: for associated functions (question: for assoc. functions that aren't methods? methods have . right?)

#### Chapter 6 - Enums, pattern matching

- enum: you can say a value is one of a possible set of values. an anum calue can only be one of its variants not multiple
- name of each variant becomes a function that constructs :: an instance. function call returns an instance
- can put any kind of data in an enum, incl. structs and other enums
- enums can have methods using impl as well
- enum **Option** as part of the standard library handles the case where  a value could be something or nothing. if something you request isn't valid you'll get nothing
- Rust does not have null to prevent errors. one of the common issues is when it is assumed a value is not null when it actually is. it uses Option<T> where the varianes are Some(T) T as in type or None
- convert Option<T> to a T before you can perform T options with it e.g. can't do 


```
fn main() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y;
}
```

- have to explicitly handle the case when a value is null
- use **match** as control flow construct with enums to handle variants, Some(T), None, inner T, no T available

#### The match control flow construct

- use match to compare value against a series of patterns. can use when enum values have data inside them and you want to extract and use those values
- difference with if is with if expression returns a boolean. here it can return any type
- match syntax `pattern => code to run`
- when match executes, compares value against pattern of each arm in order then executes code of matching arm.

> Matches in Rust are exhaustive: we must exhaust every last possibility in order for the code to be valid. Especially in the case of Option<T>, when Rust prevents us from forgetting to explicitly handle the None case

- match has a catch all pattern some variable name
- can use _ a special pattern that matches any value but does not bind to that value (don't bring that value into the match arm to be used)

- use `if let` if you just want to handle a Some(T) without having to write a handle case for None (i.e. `_ => ()`). else is like the _ case if you want but don't have to use
- if let is syntax sugar for match, runs code when the value matches one pattern and ignores all other values

#### Chapter 7 Project Management - packages, crates, modules

- useful to organize code, group functionality, separate code
- Packages (one or more crates set of functionality, min 1 crate max 1 library crate)
- Crates (binary and library)
- Modules
- Paths

- src/main.rs - crate root part of module called crate
- src/lib.rs - library crate, crate root too part of module called crate
- src/bin - put your binary crates here

- use, pub, mod  declarations, namespace, idiomatic shortcuts

#### Chapter 8 - Collections
- also stored on heap, meaning data does not need to be known at compile time, can grow or shrink
- Vector - values next to each other in memory in a single data structure. all the same type. useful for lists Vec<T>
- String
- hash map aka map

diff from tuple and array

- can use get or [] index reference. using get gives you an Option <&T> that can be used with match

```
   let v = vec![1, 2, 3, 4, 5];

    let does_not_exist = &v[100]; //panic
    let does_not_exist = v.get(100); //returns None
```

i wish there were more explanations about how things work in memory. this was a good one

error here:
```
 let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    v.push(6);
    println!("The first element is: {}", first);
```
 > This error is due to the way vectors work: because vectors put the values next to each other in memory, adding a new element onto the end of the vector might require allocating new memory and copying the old elements to the new space, if there isn’t enough room to put all the elements next to each other where the vector is currently stored. In that case, the reference to the first element would be pointing to deallocated memory. The borrowing rules prevent programs from ending up in that situation.


**Hash maps**

If we insert references to values into the hash map, the values won’t be moved into the hash map. The values that the references point to must be valid for at least as long as the hash map is valid.

- common hash actions:
- replace value with existing key
- check if key exists if so keep as is, if not insert key and value. use `entry`and `or_insert` method
- look up a key and update it based on old value, e.g.

```
  use std::collections::HashMap;
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
```
prints {"world": 2, "hello": 1, "wonderful": 1}


- strings are a collection of bytes. they are actually a vector of byets with some stringy characteristics
- string concat use `+` or `format!`
- push_str doesn't take ownership. as i found out in ch 4
- question: why can't you add two String values together, smoething to do with memory
- coercion vs deref coercion - rust can coax a &String into being a &str to use the add function

- strings have characters that also have byte representations, and so when you work with them know what you're working with 

- can use char() vs bytes()

remaining chapters to go through:
#### Chapter 9 - error handling
- panic! prints a failure, unwinds, cleans up the stack, and quits. can also abort which doesn't clean up cause it takes long
- buffer overread e.g. attempt to access element in vector that doesn't exist at v[100]
- rust has a backtrace tell you functions and how to troubleshoot
- call Result<T,E> is an enum for Ok(T) and Error(E) variants
- ? as shortcut if the return type is compatible
- ? expects Result or Option (Some/None)
- error propagation - pushing (propagating) the error somewhere else: handling error outside of calling code, and then returning the result there 
- when to use panic vs result:
- panic in test environments
- unwrap, expect are shortcuts for panic

#### Chapter 10 - generic types, traits, lifetimes
- generics include Option, Vec, Hashmap, Result where values are unknown though in a controlled way once compiled and running

#### Chapter 11 - automated tests
- assert!, assert_eq!, assert_neq!, Result<T,E>
- cargo test
- unit tests - units of code
- integration tests - test if many parts of your library work together correctly. units could work on their own but have problems when integrated

#### Chapter 13 - functional programming: iterators and closures


#### Chapter 19 - advanced features (unsafe rust, traits, types, functions, closures, macros)






## OTHER NOTES

### RUST IN ACTION

closure - anonymous or lambda

stderr error logs
stdoutput normal print logs

fields variable is annotated with the type `Vec<_>`. Vec is shorthand for _vector_, a collection type that can expand dynamically. The underscore (_) instructs Rust to infer the type of the elements.

`if let Ok(length) = fields[1].parse ::<f32>()` reads as “attempt to parse fields[1] as a 32-bit floating-point number and, if that is successful, then assign the number to the length variable.


Dangling pointers—Live references to data that has become invalid over the course of the program (see listing 1.3)

Data races—The inability to determine how a program will behave from run to run because external factors change (see listing 1.4)

Buffer overflow—An attempt to access the 12th element of an array with only 6 elements (see listing 1.5)

Iterator invalidation—An issue caused by something that is iterated over after being altered midway through (see listing 1.6)

### PERSONAL

static vs dynamically typed - rust is static, so that means you must know all of the variables at compile time and what their type is definitively and can only be one type. in dynamic languages javascript you don't necessarily know what to expect at runtime, variable types can change, not as performant, also less ideal for safety and memory reasons. benefit is that it's more efficient to code, dealing less with syntax and more with logic, but less proper. you have to be explicit in declaring types. that way will know everytime that variable is used can tell if its valid or not already

expression based language - statements such as in C and ruby can evaluate to a value, but in rust statements dont return a value, and expressions do and it makes that very distinct

> Statements are instructions that perform some action and do not return a value.

> Expressions evaluate to a resulting value.

functional vs other types of programming - in functional data is immutable, focuses more on the functions, doesn't have control flow with loops and conditionals, execution and order of statements can be important.

compiled vs interpreted language - interpret is like translating on the fly, compile is building the program in an assembly language that can be understood

having worked in other languages for web development, systems cares much more about performance, different from approach of using programming for data analysis/business logic

- rust is a static strongly typed language - static - check the types and look for errors during compile time before running program. dynamic you find out when running. strong - checks types and enfroces you use them, need to convert mix types to use together. can't mix or match types like in JS. e.g. rust once you assign a var a type can't change 

- looking at methods or function signatures helps you understand what's going on, esp with strings

### Ways to get help 
- stack overflow
- https://users.rust-lang.org/
- linter
- zulip mentor
- discord https://discord.com/channels/442252698964721669/448238009733742612
- reddit https://www.reddit.com/r/learnrust/ or rust
- https://cheats.rs/

- still lingering question: enum arms are exhastive, can you use none as an option