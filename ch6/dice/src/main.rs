fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        whatevs => move_player(whatevs),
    }
    // ok so rust knows to match matches dice_roll var to dice_roll in match, knows to move 9 into other. a catch all pattern

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {
        println!("Move player {} spaces", num_spaces);
    }

    // _ for catch all without binding, 9 isn't brought into _
    // let dice_roll = 9;
    // match dice_roll {
    //     3 => add_fancy_hat(),
    //     7 => remove_fancy_hat(),
    //     _ => reroll(),
    // }

    // fn add_fancy_hat() {}
    // fn remove_fancy_hat() {}
    // fn reroll() {}
}
