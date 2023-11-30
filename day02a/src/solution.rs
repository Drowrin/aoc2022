use itertools::Itertools;

enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    fn from(v: &str) -> RPS {
        match v {
            "A" => RPS::Rock,
            "B" => RPS::Paper,
            "C" => RPS::Scissors,
            "X" => RPS::Rock,
            "Y" => RPS::Paper,
            "Z" => RPS::Scissors,
            _ => panic!("Invalid input: {v}"),
        }
    }

    fn score(self, other: RPS) -> i32 {
        match self {
            RPS::Rock => {
                1 + match other {
                    RPS::Rock => 3,
                    RPS::Paper => 0,
                    RPS::Scissors => 6,
                }
            }
            RPS::Paper => {
                2 + match other {
                    RPS::Rock => 6,
                    RPS::Paper => 3,
                    RPS::Scissors => 0,
                }
            }
            RPS::Scissors => {
                3 + match other {
                    RPS::Rock => 0,
                    RPS::Paper => 6,
                    RPS::Scissors => 3,
                }
            }
        }
    }
}

pub fn solution(input: &str) -> String {
    input
        .split("\n")
        .map(|line| {
            let (them, me) = line.split(" ").map(RPS::from).collect_tuple().unwrap();
            me.score(them)
        })
        .sum::<i32>()
        .to_string()
}
