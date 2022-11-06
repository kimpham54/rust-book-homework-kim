fn main() {
    // `n` will take the values: 1, 2, ..., 100 in each iteration
    // let mut i = 55;
    // for j in 0..3{
    //     i+=10;
    //     println!("dog{}yooo{}",j,i);
    // }
    // println!("dog{}",i);

    // let x = 5;
    // let x = x + 2;
    // println!("{}",x);
    // shadowing, 7

    // let mut x = 5;
    // x = x + 2;
    // println!("{}",x);
    // 7

    // let mut x = 5;
    // x = x + 2;
    // println!("{}",x);
    // 7

    // let mut x = 5;
    // x = &x + 2;
    // println!("{}",x);
    // 7

    // let x = 5;
    // let x = &x + 2;
    // println!("{}",x);
    // 7

    // let x = 5;
    // &mut x = 1;
    // invalid assignment

    // let mut x = 5;
    // *&mut x = 1;
    // println!("{}",x);
    // 1

    // let x = 5;
    // let y = x;
    // println!("{}{}",x,y);
    // // works as Copy

    // let x = String::from("uuu");
    // let y = x;
    // println!("{}{}",x,y);
    // doesn't work Move
}

// fn main() {
//     let mut names = 3;

//     for i in 0..3{
//         names = i + 15
//     }

//     println!("names: {:?}", names);
// }

fn main() {
    // `n` will take the values: 1, 2, ..., 100 in each iteration
    // let mut i = 55;
    // for j in 0..3{
    //     i+=10;
    //     println!("dog{}yooo{}",j,i);
    // }
    // println!("dog{}",i);

    // let x = 5;
    // let x = x + 2;
    // println!("{}",x);
    // shadowing

    // let mut x = 5;
    // x = x + 2;
    // println!("{}",x);

    // let mut x = 5;
    // x = x + 2;
    // println!("{}",x);

    // let mut x = 5;
    // x = &x + 2;
    // println!("{}",x);

    // let x = 5;
    // let x = &x + 2;
    // println!("{}",x);

    // let x = 5;
    // &mut x = 1;

    // let mut x = 5;
    // *&mut x = 1;

    // why does this work
    // let x = 2;
    // println!("{x}", x = x+2);

    let country = String::from("Austria");
    let country_ref = &country;
    let country = 8;
    println!("{}, {}", country_ref, country);
}

// fn main() {
//     let mut names = 3;

//     for i in 0..3{
//         names = i + 15
//     }

//     println!("names: {:?}", names);
// }

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            let largest = number; //original code didn't have let
        }
    }

    println!("The largest number is {}", largest);
}
// largest stays as 34 cause let largest = number is inner scope shadowing
