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

// + std::fmt::Display + std::fmt::Debug

impl<'a> Sum<&'a Coin> for u8 {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Coin>,
    {
        let mut results = Vec::new();

        // if Coin == Coin::Penny, push 25

        for coin in iter {
            match coin {
                Coin::Penny => results.push(1),
                Coin::Nickel => results.push(5),
                Coin::Dime => results.push(10),
                Coin::Quarter(state) => {
                    println!("State quarter from {:?}!", state);
                    results.push(25);
                }
            }
        }

        // for item in iter {
        //     match item {
        //         Coin::Penny => 1 as u8,
        //         Coin::Nickel => 5,
        //         Coin::Dime => 10,
        //         Coin::Quarter(state) => {
        //             println!("State quarter from {:?}!", state);
        //             25
        //         }
        //     };

        // results.push(item);
        println!(
            "Results of Sum Trait for an Iterator of Coins {:?}",
            results
        );

        let sum: u8 = results.iter().fold(
            0,
            // #[rustc_inherit_overflow_checks]
            |a, b| a + b,
        );
        sum
    }
    //return a u8 value for now so that .sum works
    //use this once i've converted the coins to u8s
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
    let change = vec![Coin::Penny, Coin::Penny, Coin::Penny, Coin::Dime];

    // for loop way
    // let mut results: u8 = 0;
    // for item in change {
    // results += value_in_cents(item);
    // println!("{:?}", results);
    // }

    let mut list = Vec::new();
    for item in change {
        list.push(value_in_cents(item));
        println!("{:?}", &list);
    }
    let looplist: u8 = list.iter().sum();
    println!("for loop traditional way sum is {:?}", looplist);

    let morechange = vec![
        Coin::Penny,
        Coin::Penny,
        Coin::Penny,
        Coin::Penny,
        Coin::Dime,
    ];
    let sum: u8 = morechange.iter().sum();
    println!("the final balance using the sum trait is {}", sum);

    let evenmorechange = vec![200, 3, 4];
    let total: u8 = evenmorechange.iter().sum();
    println!("{:?}", total);
}
