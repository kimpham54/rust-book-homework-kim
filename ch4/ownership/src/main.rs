fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);

    // can have unlimited immutable references

    // let mut s = String::from("hello");

    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // let r3 = &s; // BIG PROBLEM

    // println!("{}, {}, and {}", r1, r2, r3);

    // can't borrow simultaneous mutable references to the same var and use it same time without finishing use

    // fn main() {
    //     let mut s = String::from("hello");

    //     let r1 = &mut s;
    //     let r2 = &mut s;

    //     println!("{}, {}", r1, r2);
    // }
}
