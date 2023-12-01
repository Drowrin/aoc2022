use itertools::Itertools;

enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn from(v: &str) -> Shape {
        match v {
            "A" => Shape::Rock,
            "B" => Shape::Paper,
            "C" => Shape::Scissors,
            "X" => Shape::Rock,
            "Y" => Shape::Paper,
            "Z" => Shape::Scissors,
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

fn parse_line(line: &str) -> (Shape, Shape) {
    line.split(" ").map(Shape::from).collect_tuple().unwrap()
}

fn get_score((them, me): (Shape, Shape)) -> i32 {
    me.val()
        + match me {
            Shape::Rock => match them {
                Shape::Rock => 3,
                Shape::Paper => 0,
                Shape::Scissors => 6,
            },
            Shape::Paper => match them {
                Shape::Rock => 6,
                Shape::Paper => 3,
                Shape::Scissors => 0,
            },
            Shape::Scissors => match them {
                Shape::Rock => 0,
                Shape::Paper => 6,
                Shape::Scissors => 3,
            },
        }
}

pub fn solution(input: &str) -> impl ToString {
    input
        .split("\n")
        .map(parse_line)
        .map(get_score)
        .sum::<i32>()
}
