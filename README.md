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
- unwrap, expect are shortcuts for panic. unwrap_or_else(the_or_else_goes_in_here)

#### Chapter 10 - generic types, traits, lifetimes
- generics include Option, Vec, Hashmap, Result where values are unknown though in a controlled way once compiled and running
- make functions generic by extracting to remove code duplication, paramterizing types
- When you recognize situations in your code with multiple struct or enum definitions that differ only in the types of the values they hold, you can avoid duplication by using generic types
- can have generic methods on structs and enums,
```
struct Point<T> {
    x: T,
    y: T,
}
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
fn main() {
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
}
```
- a trait is a generic or abstract method
- declares types after impl using impl<T> flexible to use on diff types
- traits can have multiple methods. each type that uses the trait must provide its own custom behaviour in the body of the method
- can gain methods implicitly by defining traits that call other traits
- default implementations, overriding implementations
- traits can be parameters too, pass in any type that has that trait
- trait bound set restrictions, if you want to constrain function to the same or different types can do so. with impl trait or with trait bound syntax <T: summary etc>
- where clauses for cleaner function signatures
- can return a value of a type that implements a trait if impl Trait is in the return position of a function signature. returns a type. useful for closures and iterators. can only return 1 type of that trait. multiple options not allowed
- most of the time inferred like types. annotate when multiple types are possible, similarly annotate when lifetimes of refernces could be related in diff ways ensure its valid
- annotate lifetimes for functions
- 'a lifetime symbol denotation for borrow checker to check. means sshortest of the 2, doesn't change a lifetime and doesn't need to know exact lifetime . where lifetimes overlap
- goes into function signature
- string literals are refernces, use lifetimes for references

3 rules for lifetimes
1. all references get their own lifetime
2. if one input the same lifetime gets assigned
3. self gets same lifetime as output


#### Chapter 11 - automated tests
- assert!, assert_eq!, assert_neq!, Result<T,E>
- cargo test
- unit tests - units of code
- integration tests - test if many parts of your library work together correctly. units could work on their own but have problems when integrated

- a test is a function annotated with a test attribute
- you can run tests in parallel or consecutively. careful if parallel and tests conflict with one another - so this can happen if you're overwriting a file. ok but if you're doing parallel tests like add to a value it's taking the original value not adding up what the new value might be, e.g. value of a isn't changing based on each test kim

```
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }
}
```

#### Chapter 13 - functional programming: iterators and closures

#### Closures
- Closures can capture values from their environment in three ways, which directly map to the three ways a function can take a parameter: borrowing immutably, borrowing mutably, and taking ownership. The closure will decide which of these to use based on what the body of the function does with the captured values.
- from 13.03 The functional programming style prefers to minimize the amount of mutable state to make code clearer. Removing the mutable state might enable a future enhancement to make searching happen in parallel, because we wouldn’t have to manage concurrent access to the results vector.

```
def dog(x):
    return x[1]
# sort by the index value 1 ascending
  
a = [(1, 2), (3, 1), (5, 10), (11, -3)]
a.sort(key=dog) 

print(a[1])
```

- sound a lot like lambdas, anonymous functions, pure functions
- functions, everything a turing machine


- compiler infers type in closure if not explicit, after which it cannot be changed
- closure body assumes a reference without the & ampersand, because it's the least amount of access needed. use move to explicitly transfer ownership. in some cases you would like spawning threads

Closure capture values from their environment in the same way functions take a parameter:
- borrows immutably &, except you don't need the &
- borrows mutably &mut, except you don't need the &
- transfers ownership let

Three types of traits of closures:
- Fnonce - call a closure once, all closures implement at least this. can move captured values out of its body
- FnMut - don't move captured values out of body, mutate possible. called more than once
- Fn - don't move captured values out of their body, don't mutate captured values, or capture nothing from their env. can be called more than once without mutating their env

- to use closures properly you have to think about the closure environment, how often it is using the values/variables in that environment, whether or not they are moving out and if you are trying to call them again if they've already moved out. hard! see listing 13-7,8,9

#### Iterators
- iterators are a pattern on a vector Vec<T>, with an Iterator trait, associated types are Item and self::Item. methods include next, iter, etc.
- are lazy (don't do anything unless you consume the iterator using methods like next sum collect)
- iterators are a trait, .iter()
- requires the object to define a next method, returning one item wrapped in Some and when iteration is over returns None
- iterators requires mutability cause it uses up the iterator. for loops take ownership and makes it mutable behind the scenes. gaaah
- .iter, .into_iter (takes ownership), .iter_mut
- methods like .next, .sum, .collect are consuming adaptors, they consume the iterator as it goes through it, taking ownership. can't call it after the use it's goooone
- methods like map are iterator adaptors
- .filter creates a new iterator (of the filtered results), map and collect does too

- high level ideas at low level performance, runtime performance not affected, not sure how to explain why. lazy runs only when it needs to, compiles to same code almost, loop code is efficient

#### Chapter 19 - advanced features (unsafe rust, traits, types, functions, closures, macros)
- unsafe rust
- good to wrap unsafe code in a safe abstraction

- a raw pointer is a pointer whose lifetime isn't controlled by an encapsulating object, e.g. can be assigned the address when nothing is there, explicit pointing to an address in memory not smart where it optimizes the place to point to in memory for you
*const T and *mut T are raw pointers
- dereference with * meaning the following:
```
 let mut x = 100;
    let y = &mut x;
    *y += 100;
    // dereference means y gets the value of x and is no longer a reference
    // dereference means that you gave it back to x?
    let z = &mut x;
    *z += 1000;
    assert_eq!(x, 1200);
    ```
it's confusing and i keep getting confused. see https://www.koderhq.com/tutorial/rust/smart-pointer/#:~:text=The%20dereference%20operator%20is%20also,behaviour%20of%20the%20dereferencing%20operator. for another comparison example under Deref<T> https://doc.rust-lang.org/std/ops/trait.Deref.html. my own confused experiments https://replit.com/@kimpham54/MintcreamDismalQuote#src/main.rs

- associated type







































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

- semantic assignment difference between binding and assignment
- https://stackoverflow.com/questions/55204816/what-is-the-meaning-of-the-error-left-hand-of-expression-not-valid
- https://stackoverflow.com/questions/47648958/whats-the-semantic-of-assignment-in-rust/47649554#47649554

### Ways to get help 
- stack overflow
- https://users.rust-lang.org/
- linter
- zulip mentor
- discord https://discord.com/channels/442252698964721669/448238009733742612
- reddit https://www.reddit.com/r/learnrust/ or rust
- https://cheats.rs/
https://dhghomon.github.io/easy_rust/Chapter_17.html
- Rust by Example
- Rust Reference
https://blog.boot.dev/rust/variable-shadowing-in-rust/
https://blog.thoughtram.io/string-vs-str-in-rust/
- pascal precht

- still lingering question: enum arms are exhastive, can you use none as an option


#### lambda calculus
lambda functions
- map reduce hadoop. map in ruby uses lambdas
- python uses maps and lambdas too

dog = map(lambda x: x * 2, [1,2,3,4])
print(list(dog))

- nice for readability and shortcuts
- anonymous functions that are good for security reasons too. you use and dump right away, it can't be inherited
- lambda functions good for map and reduce type functions where you want to apply the function to a given list, filter. map - where you want to call the same function over and over on something like a list
- lambda functions safer throwaway functions apply to list and not do anything else with it can't call/reuse function elsewhere in a dangerous way. it's not a function that can be accessed as easily or inherit something bad. 
enum

- loops are in functions
- lambdas are good for looping entire functions
- lambdas - good for calling a function over and over on something. map, filter, reduce

```
# dog = map(lambda x: x * 2, [1,2,3,4])

def dog(x):
  return x*2

cat = dog(3)

# cat2 = [1,2,3,4]

# calling the dog function on the list 
cat3 = map(dog, [1,2,3,4])

# useful here, just one line
chicken = map(lambda x:x*2, [1,2,3,4])
```

```
def dog(x):
    return x[1]
# sort by the index value 1 ascending
  
a = [(1, 2), (3, 1), (5, 10), (11, -3)]
a.sort(key=dog) 

print(a[1])
```

best - https://stackoverflow.com/questions/47407180/do-lambda-expressions-have-any-use-other-than-saving-lines-of-code
ok explanation - https://www.geeksforgeeks.org/python-lambda-anonymous-functions-filter-map-reduce/
ok - https://python-course.eu/advanced-python/lambda-filter-reduce-map.php
good code - https://stackoverflow.com/questions/3259322/why-use-lambda-functions
ok - https://stackoverflow.com/questions/890128/how-are-lambdas-useful

- something about enums? static/dynamic types?
- can create values of anonymous types in some programs

not bad video - https://www.youtube.com/watch?v=m32kbFXBRR0&t=148s
- https://www.steveclarkapps.com/lambda-calculus/
- http://morphett.info/turing/turing.html
- https://brilliant.org/wiki/turing-machines/

- all programming all computers are functions
- functions take something and transforms it. has a transition state
- all programming is about manipulating variables
- https://brilliant.org/wiki/lambda-calculus/
- variable binding and substitution
- binding means let, new, comes into scope
- substitition means...something like assignment but implicit apparently
- assignment (existing variable come in)
https://cs.stackexchange.com/questions/126587/difference-between-assignment-binding-and-substitution
- statements vs expressions



# benefits of rust
- references make it safe
- strong, static types
- prevent data races at comppile time (ch 4.2)
- memory safety
- avoids null references, all references are always valid
- buffer overflow/buffer overrun - dangling pointers, memory not deallocated
- 80% or vast majority wired mag says these issues caused by these languaes. borrow checker. no straight pointers and references. immutable types by default, variables change only if explictly telling it can be mutable. borrow checker checks memory and that you did this properly
- null pointer dereferencing
- danging references



- idempotent insanity albert einstein
- polymorphism
- question: structs and enums are data structures not types, and they are hetrogeneous?
- blockchain merkle tree hash tree, linked list



# TERMS TERMS TERMS TERMS TERMS 
## Chapter 1
- linker compiler
- ahead-of-time compiled language
- prelude - e.g. set of items defined in standard libraires that brings into scope of every program
- monomorphization

## Chapter 2
- immutable/mutable
- library crate
- registry
- mismatched types
## Chapter 3
- keywords
- shadowing - often used to convert one type to another type. when you redeclare a variable
- scalar type - single values
- integer overflow
- two's complement wrapping
- compound types
- snake_case not camelCase
- expressions, statements.
- statements (2 kinds declaration letstatement, expressionstatement where it ignores the result see rust reference)
- expressions (many see https://doc.rust-lang.org/reference/expressions.html)
- match, arms
## Chapter 4
- garbage collection
- pointer
- allocating on the heap, memory allocation
- owner
- scope
- drop - deallocating resources at the end of a lifetime
** aka resource acquisition is initialization RAII
- double free error
- invalidated reference
- shallow copy
- deep copy
- move
- automatic copy
- clone
- copy
- references
- dereferencing
- borrowing - the action of creating a reference
- mutable/immutable reference
- data race
- non lexical lifetimes NLL
- dangling pointer, dangling references
- slices are references, always being borrowed
- deref coercions


- utf8
- memory leak
## Chapter 5
- struct, fields
- instance
- struct update syntax
- tuple like struct
- unit like struct
- lifetimes
- macro
- method impl
- getters
- automatic referencing and dereferencing
- associated function - functions defined within an impl block
- constructors
## Chapter 6
- enum: useful if you want to know what kind of data you have and store the data. each variant can have diff types and amountsof associated data vs struct.
- variant
- Option<T> -> Some<T> or None is a generic. Some returns T. in chapter 13.1 i think you can be flex about what Some/None returns
- there is no null, but None exists in option. () is unit type
- match, arms. matches are exhaustive
- _ catch0all pattern
- if let - shortcut for match, less boilerplate code, syntax sugar, use when value matches one pattern and ignores all other values
- enums have methods too
## Chapter 7
- workspace
------------------
-The module system

- crates - smallest amount of code - binary or library crate
- binary: pexecutable programs you can run, has main function
- library: no main function, doesnt compile to an executable, functionality shared with multiple projects
- crate root - source file that compiler starts from and makes up root module

- packages - one or more creates set of functionality, e.g. cargo. can contain as many binary crates, 1 library crate. min 1 crate

- modules, submodules modules let us organize code within a crate for reuse. control privacy with public/private. private by default
- paths, absolute from crate root vs self, super for relative
- structs by default all private
- enums once public all public
- use keyword to make valid/invalid bring into scope. parent/child
- re-exporting module
---------------------
- scope - managed by this organization
- idiomatic
- encapsulation
- nested context
- alias
## Chapter 8
- collections are scalable not determined size. data stored on the heap.
- vector Vec<T> is homogenous store values same type
- String (not string literal, not &str/str string slice) - implemented as a wrapper around a vector of bytes with some xtra guarantees, restrictions, capabilities
- reference/deref coercion see Ch 15
- UTF-8 encoded
- scalar values, grapheme clusters, bytes byte representation to store characters aka Unicode scalar values

- hash map - hashing function, like a dictionary, stores a mapping of keys to values. next to each other in memory and determines how it places these. homogenous data keys same, values same type
- SipHash hasher
- talks a bit more about function signatures and push_str exactly my confusion before
## Chapter 9
- recoverable and unrecoverable errors
- panic!
- unwinding, aborting
- buffer overread - read beyond a data structure
- backtrace - list all functions have been called up until this point to troubleshoot
- Result<T,E> => Ok<T>, Err<E> is a generic
- unwrap - shortcut like match. use for Result -> OK or Err will return panic!
- expect -> use instead of unwrap, Ok and then if Error includes custom message with panic!
- propagating the error - handling the error where you want it to not necessarily where it happens. spread or move it forward use ? shortcut instead for syntax sugar, replaces Result, OK, Err
- bad state vs. expected failure
- contracts - functions have contracts, their behaviour guarnateed if inputs meet requirements

- function signature
## Chapter 10
- generics - abstract stand ins for concrete types
- traits - to constrain generic types to a behaviour
- lifetimes - variety of generics that give compiler info about how references relate to each other
- Monomorphization, there is no cost to using generics than with concrete types
- trait, known as interfaces in other languages with some differences. groups method signatures together to define a set of behaviours to accomplish some purpose
- trait bounds
- coherence restriction property, orphan rule. where traits cannot be implemented on a type that isn't local to your crate code
- blanket implementations
- compile time checking
- lifetimes - every reference in rust has a lifetime, which is the scope for which that reference is valid
- dangling references - use lifetimes to prevent a program to reference data other than its intended reference
- borrow checker (the infamous) - compares scopes to determine whether all borrows are valid
- lifetime annotation syntax
- lifetme elision rules
- input/output lifetimes
- dangling references
- static lifetime


## Chapter 11
- assert! for testling left = something or whatever comparison
- assert_eq! for testing if left and right of the == are equal. left == right
- assert_neq! useful if you don't know what the value should be but you know what it shouldn't be, so left !== right
- derive attribute on a trait to get functionality on a type to apply to, i think....
- format! macro
- #[should_panic] to be less precise in our test, can use expected to troubleshoot more
- Result<T, E>
- assert!(value.is_err())
- unit tests go in src
- integration tests go in your test directory, functions need to be public
- files in tests don't behave in the same way as src, might need to add mod.rs for helper functions or extra code not an integration test file
- integration tests can only test in lib.rs file. main.rs is generally simple
- tests go in lib.rs

## Chapter 13
- linked lists not next to each other in memory, slow
- array vs vector
- array - stack
- vector - heap, dynamic, growable, more methods. size can change in runtime (so can shrink be less). array always same capacity
- array vector both contiguous storage in memory
- https://stackoverflow.com/questions/60583618/performance-of-rust-vector-vect-versus-array-t-n
- array slice reference to an array


- closures are a function like construct you can store in a variable. a function that can be called like a variable. like lambdas/pure functions but it never said that
- iterators, a way of processing a series of elements. iterator objects. not like enums enums for matching
- lazy
- associated type
- consuming adaptor - iterator traits that consume, like next and sum, eats it up as it goes through the iterator object and takes ownership
- iterator adaptor - doesn't consume the iterator, .map creates a new iterator as it goes through

- closures and iterators functional programming. is as fast as lower level (less abstraction) loops
- less code, simpler, less mutable vars introduced
- zero cost abstractions (such as iterators, high level abstraction but compiled as if lower level)
- zero-overhead - In general, C++ implementations obey the zero-overhead principle: What you don’t use, you don’t pay for. And further: What you do use, you couldn’t hand code any better.
- unrolling (optimization that removes overhead of the loop controlling code and instead generates repetivie code for each iteration)
- registers (coefficients in video buffering get stored in there, makes fast)

## Chapter 19
- raw pointers
- print pointers, printf!("{}", ptr as usize) or println!("{:p}", &x);