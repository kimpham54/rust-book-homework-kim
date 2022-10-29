fn main() {
    fn first_word(s: &String) -> &str {
        let bytes = s.as_bytes();
        println!("you are here");

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                println!("word loop {}", &s[0..i]);
                break;
                // return &s[0..i]; returns function before printing rest
            }
        }
        println!("print me");
        println!("word {}", &s[..].to_string());
        &s[..]
    }

    first_word(&String::from("time for some tea"));

    // fn first_word(s: &String) -> usize {
    //     let bytes = s.as_bytes();
    //     for (i, &item) in bytes.iter().enumerate() {
    //         if item == b' ' {
    //             return i;
    //         }
    //     }
    //     s.len()
    // }

    // my own test works
    let s = String::from("yoyo");
    let hey = s.as_bytes();
    println!("{:x?}", hey);

    let mut pity = String::from("ice box");
    let word = first_word(&pity);
    println!("the first word is: {}", word);
    // was getting error before adding :x? for hexademical integer https://stackoverflow.com/questions/27650312/show-u8-slice-in-hex-representation
}
