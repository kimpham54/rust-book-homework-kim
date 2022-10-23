fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                println!("inner loop");
                break;
            }
            if count == 2 {
                println!("outer loop");
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

// count = 0
// remaining = 10
// remaining = 9
// inner loop
// count = 1
// remaining = 10
// remaining = 9
// inner loop
// count = 2
// remaining = 10
// outer loop
// End count = 2
