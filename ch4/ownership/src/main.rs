fn main() {
    //     let s1 = String::from("hello");

    //     let len = calculate_length(s1);

    //     println!("The length of '{}' is {}.", s1, len);
    // }

    // fn calculate_length(s: String) -> usize {
    //     let length = s.len(); // len() returns the length of a String

    //     length
    // this doesn't work because the s1 is value is borrowed after the move, need to use reference &s1

    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
