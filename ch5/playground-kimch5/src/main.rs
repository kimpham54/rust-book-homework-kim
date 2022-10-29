fn main() {
    // x borrowed no work to use x
    // let x = String::from("girl");
    // let y = x;
    // println!("x = {}, y = {}", x, y);

    // works cause reference read only, x=girl, y=girl
    // let x = String::from("girl");
    // let y = &x;
    // println!("x = {}, y = {}", x, y);

    // x = papafriend
    // let mut x = String::from("girl");
    // x = String::from("papa");
    // x.push_str("friend");
    // println!("x = {}", x);

    //  x = girlfriend, y = (). push_str() returns (). x is not invalidated
    // let mut x = String::from("girl");
    // let y = x.push_str("friend");
    // println!("x = {:?}, y = {:?}", x, y);

    // returns foo, 3 - you're not actually borrowing the value i guess, you're doing osmething with it and getting a new result. always think about the pointers and memory relation for it to make sense
    // let mut s = String::from("foo");
    // let y = s.capacity();
    // println!("{},{}", s, y);

    //  x = girlfriend, y = (). push_str() returns ()
    // let mut x = String::from("girl");
    // let y = &x.push_str("friend");
    // println!("x = {:?}, y = {:?}", x, y);
}
