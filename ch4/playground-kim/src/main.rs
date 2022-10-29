fn main() {
    // let s1 = String::from("hello");
    // let (s2, len) = calculate_length(s1);
    // println!("The length of '{}' is {}.", s1, len);

    // fn calculate_length(s: String) -> (String, usize) {
    //     let length = s.len();
    //     (s, length)
    // }

    // let x = 5;
    // let y = 2;
    // let z = x + 10;

    // fn plus_one(x: i32) -> i32 {
    //     x + 1
    // }
    // println!("{}, {}, {}", x, y, z);

    // shadowing doesn't destroy a value but blocks it
    // let mut country = 9;
    // let country_ref = &country;
    // let country = country + 5;
    // println!("{}, {}", country_ref, country);

    // returns austria, 8. doesnt' destroy first country
    // let country = String::from("Austria");
    // let country_ref = &country;
    // let country = 8;
    // println!("{}, {}", country_ref, country);

    // let a = [1, 2, 3, 4, 5];
    // let slice = &a[1..3];

    // no error
    // let x = 5;
    // let y = x;
    // println!("x = {}, y = {}", x, y);

    // error ownership change
    // let x = String::from("hello");
    // let y = x;
    // println!("x = {}, y = {}", x, y);

    // this is ok
    let x = String::from("hello");
    let y = &x;
    println!("x = {}, y = {}", x, y);

    // no error
    // let x = [1, 2, 3, 4, 5];
    // let y = x;
    // println!("x = {:?}, y = {:?}", x, y);
}
