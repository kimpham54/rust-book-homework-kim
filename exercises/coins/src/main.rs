use core::iter::Sum;
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl<'a> Sum<&'a Coin> for u8 {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Coin>,
    {
        let sum: u8 = Self::Item;
        sum
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    let change = vec![
        Coin::Penny,
        Coin::Penny,
        Coin::Penny,
        Coin::Quarter(UsState::Alaska),
    ];

    let mut results: u8 = 0;

    // for loop way
    // let mut results = Vec::new();
    // for item in change {
    // results += value_in_cents(item);
    // println!("{}", &results);
    // // results.push(value_in_cents(item));
    // }

    let sum: u8 = change.iter().sum();
    println!("the final balance is {}", sum);

    // let dog = Coin::Penny;
    // let cat = value_in_cents(Coin::Quarter(UsState::Alaska));
    // println!("{}, {}", dog, cat);
}
