use std::cmp::Ordering;

#[derive(Debug)]
#[derive(PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors
}

#[derive(Debug)]
struct Round {
    theirs: Shape,
    mine: Shape
}

impl PartialOrd for Shape {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match (self, other) {
            (Shape::Rock, Shape::Paper) => Ordering::Less,
            (Shape::Rock, Shape::Scissors) => Ordering::Greater,
            (Shape::Paper, Shape::Rock) => Ordering::Greater,
            (Shape::Paper, Shape::Scissors) => Ordering::Less,
            (Shape::Scissors, Shape::Rock) => Ordering::Less,
            (Shape::Scissors, Shape::Paper) => Ordering::Greater,
            _ => Ordering::Equal
        })
    }
}

impl Round {
    fn new(s: String,
           theirs_mapper: fn(char) -> Option<Shape>,
           mine_mapper: fn(char) -> Option<Shape>) -> Option<Round> {
        let theirs_option = s.as_str().chars().nth(0).map(theirs_mapper).flatten();
        let mine_option = s.as_str().chars().nth(2).map(mine_mapper).flatten();

        match (theirs_option, mine_option) {
            (Some(theirs), Some(mine)) => Some(Round { theirs: theirs, mine: mine }),
            _ => None
        }
    }

    fn score(&self) -> i32 {
        let score_from_outcome = match (&self.mine, &self.theirs) {
            (a, b) if a < b => 0,
            (a, b) if a > b => 6,
            (_, _) => 3
        };

        let score_from_shape = match self.mine {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3
        };

        score_from_outcome + score_from_shape
    }
}
    
fn read_next_round<B>(lines: &mut std::io::Lines<B>) -> Option<Round> where B: std::io::BufRead {
    lines.next().map(|next_result|
                     Round::new(next_result.expect("Failed to read line"),
                                |c| match c {
                                    'A' => Some(Shape::Rock),
                                    'B' => Some(Shape::Paper),
                                    'C' => Some(Shape::Scissors),
                                    _ => None
                                },
                                |c| match c {
                                    'X' => Some(Shape::Rock),
                                    'Y' => Some(Shape::Paper),
                                    'Z' => Some(Shape::Scissors),
                                    _ => None
                                }
                     )
                     .expect("Failed to parse round on line"))
}

fn read_all_rounds<B>(mut lines: std::io::Lines<B>) -> Vec<Round> where B: std::io::BufRead {
    let mut rounds = Vec::new();
    while let Some(round) = read_next_round(&mut lines) {
        rounds.push(round);
    }
    rounds
}

fn main() {
    let rounds : Vec<Round> = {
        use std::io::prelude::*;
        read_all_rounds(std::io::stdin().lock().lines())
    };

    println!("{}", rounds.iter().map(|round| round.score()).sum::<i32>())
}


    // let mut end_of_file = true;
    // let mut inventory : Vec<i32> = Vec::new();
    
    // match lines.next() {
    //     Some(result) => {
    //         let line = result.expect("Failed to read line");
    //         let line_trimmed = line.trim();
    //     match line_trimmed {
    //         "" => break,
    //         _ => {
    //             let calories : i32 = line_trimmed.parse().expect("Failed to parse calories value");
    //             inventory.push(calories);
    //         }
    //     }
    // }
    // if end_of_file {
    //     None
    // }
    // else {
    //     Some(inventory)
    // }
