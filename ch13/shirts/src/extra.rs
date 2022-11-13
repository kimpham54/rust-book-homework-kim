// let mut list = vec![1, 2, 3];
// // let dog = &mut list.push(7);
// // println!("value of {:?}", list);
// how do i move list into dog? so hard gah.

// let v1 = vec![1, 2, 3];

// let v1_iter = v1.iter();

// for val in v1 {
//     println!("Got: {}", val);
// }

// let v1 = vec![7, 2, 3];

//         let mut v1_iter = v1.iter();
//         println!("{:#?}", v1_iter.next())

//     let string1 = String::from("abcd");
//     let string2 = "xyz";

//     let result = longest(string1.as_str(), string2);
//     println!("The longest string is {}", result);
// }

// fn longest<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
// result.as_str()

// fn longest<'a>(x: &str, y: &str) -> String {
//     let result = String::from("really long string");
//  result

fn main() {
    fn take(g: Vec<i32>) {
        // What happens here isn’t important.
    }

    let v = vec![1, 2, 3];

    take(v);

    println!("v[0] is: {}", v[0]);
}

fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    fn take(g: Vec<i32>) {
        // What happens here isn’t important.
    }
    take(list);
    println!("After calling closure: {:?}", list);
}
