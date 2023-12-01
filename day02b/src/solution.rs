use itertools::Itertools;

enum Shape {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Lose,
    Draw,
    Win,
}

impl Shape {
    fn from(v: &str) -> Shape {
        match v {
            "A" => Shape::Rock,
            "B" => Shape::Paper,
            "C" => Shape::Scissors,
            _ => panic!("Invalid input: {v}"),
        }
    }

    fn val(&self) -> i32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

impl Outcome {
    fn from(v: &str) -> Outcome {
        match v {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("Invalid input: {v}"),
        }
    }

    fn val(self) -> i32 {
        match self {
            Outcome::Lose => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }

    fn shape(&self, them: Shape) -> Shape {
        match self {
            Outcome::Lose => match them {
                Shape::Rock => Shape::Scissors,
                Shape::Paper => Shape::Rock,
                Shape::Scissors => Shape::Paper,
            },
            Outcome::Draw => them,
            Outcome::Win => match them {
                Shape::Rock => Shape::Paper,
                Shape::Paper => Shape::Scissors,
                Shape::Scissors => Shape::Rock,
            },
        }
    }
}

fn parse_line(line: &str) -> (Shape, Outcome) {
    let (shape, outcome) = line.split(" ").collect_tuple().unwrap();
    (Shape::from(shape), Outcome::from(outcome))
}

fn get_score((their_shape, outcome): (Shape, Outcome)) -> i32 {
    let me = outcome.shape(their_shape);
    me.val() + outcome.val()
}

pub fn solution(input: &str) -> impl ToString {
    input
        .split("\n")
        .map(parse_line)
        .map(get_score)
        .sum::<i32>()
}
