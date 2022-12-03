pub mod rps;

fn main() {
    
    let rounds : Vec<rps::Round> = {
        use std::io::prelude::*;
        rps::read_all_rounds(std::io::stdin().lock().lines(), rps::Round::from_shape_outcome_string)
    };

    println!("{}", rounds.iter().map(|round| round.score()).sum::<i32>())
}
