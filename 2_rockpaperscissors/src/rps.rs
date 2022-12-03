#[derive(Debug)]
pub enum Outcome {
    Draw = 0,
    Win = 1,
    Lose = 2
}

impl Outcome {
    fn from_u32(i: u32) -> Outcome {
        match i {
            0 => Outcome::Draw,
            1 => Outcome::Win,
            2 => Outcome::Lose,
            _ => panic!("Invalid value for outcome")
        }
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Shape {
    Rock = 0,
    Paper = 1,
    Scissors = 2
}

impl Shape {
    fn from_u32(i: u32) -> Shape {
        match i {
            0 => Shape::Rock,
            1 => Shape::Paper,
            2 => Shape::Scissors,
            _ => panic!("Invalid value for shape")
        }
    }
}

impl Shape {
    fn play(&self, rhs: &Shape) -> Outcome {
        let outcome_u32 = (3 + (*self as u32) - (*rhs as u32)) % 3;
        Outcome::from_u32(outcome_u32)
    }
}

impl Shape {
    fn infer_mine(&self, rhs: &Outcome) -> Shape {
        let shape_u32 = ((*self as u32) + (*rhs as u32)) % 3;
        Shape::from_u32(shape_u32)
    }
}

#[derive(Debug)]
pub struct Round {
    mine: Shape,
    outcome: Outcome
}

impl Round {
    fn from_shapes(theirs: Shape, mine: Shape) -> Round {
        let outcome = mine.play(&theirs);
        Round { mine, outcome }
    }

    fn from_shape_outcome(theirs: Shape, outcome: Outcome) -> Round {
        let mine = theirs.infer_mine(&outcome);
        Round { mine, outcome }
    }
    
    pub fn from_shapes_string(s: String) -> Option<Round> {
        let theirs_mapper = |c| match c {
            'A' => Some(Shape::Rock),
            'B' => Some(Shape::Paper),
            'C' => Some(Shape::Scissors),
            _ => None
        };
        
        let mine_mapper = |c| match c {
            'X' => Some(Shape::Rock),
            'Y' => Some(Shape::Paper),
            'Z' => Some(Shape::Scissors),
            _ => None
        };
        
        let theirs_option = s.as_str().chars().nth(0).map(theirs_mapper).flatten();
        let mine_option = s.as_str().chars().nth(2).map(mine_mapper).flatten();

        match (theirs_option, mine_option) {
            (Some(theirs), Some(mine)) => Some(Round::from_shapes(theirs, mine)),
            _ => None
        }
    }

    pub fn from_shape_outcome_string(s: String) -> Option<Round> {
        let theirs_mapper = |c| match c {
            'A' => Some(Shape::Rock),
            'B' => Some(Shape::Paper),
            'C' => Some(Shape::Scissors),
            _ => None
        };
        
        let outcome_mapper = |c| match c {
            'X' => Some(Outcome::Lose),
            'Y' => Some(Outcome::Draw),
            'Z' => Some(Outcome::Win),
            _ => None
        };
        
        let theirs_option = s.as_str().chars().nth(0).map(theirs_mapper).flatten();
        let outcome_option = s.as_str().chars().nth(2).map(outcome_mapper).flatten();

        match (theirs_option, outcome_option) {
            (Some(theirs), Some(outcome)) => Some(Round::from_shape_outcome(theirs, outcome)),
            _ => None
        }
    }
    
    pub fn score(&self) -> i32 {
        let score_from_outcome = match self.outcome {
            Outcome::Lose => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        };

        let score_from_shape = match self.mine {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3
        };

        score_from_outcome + score_from_shape
    }
}
    
fn read_next_round<B>(lines: &mut std::io::Lines<B>, string_to_round: fn(String) -> Option<Round>) -> Option<Round> where B: std::io::BufRead {
    lines.next().map(|next_result|
                     string_to_round(next_result.expect("Failed to read line"))
                     .expect("Failed to parse round on line"))
}

pub fn read_all_rounds<B>(mut lines: std::io::Lines<B>, string_to_round: fn(String) -> Option<Round>) -> Vec<Round> where B: std::io::BufRead {
    let mut rounds = Vec::new();
    while let Some(round) = read_next_round(&mut lines, string_to_round) {
        rounds.push(round);
    }
    rounds
}
