enum RPS {
    Rock,
    Paper,
    Scissors,
}

enum Result {
    Lose,
    Draw,
    Win,
}

impl RPS {
    fn from(v: &str) -> RPS {
        match v {
            "A" => RPS::Rock,
            "B" => RPS::Paper,
            "C" => RPS::Scissors,
            _ => panic!("Invalid input: {v}"),
        }
    }

    fn val(self) -> i32 {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }
}

impl Result {
    fn from(v: &str) -> Result {
        match v {
            "X" => Result::Lose,
            "Y" => Result::Draw,
            "Z" => Result::Win,
            _ => panic!("Invalid input: {v}"),
        }
    }

    fn shape(&self, them: RPS) -> RPS {
        match self {
            Result::Lose => match them {
                RPS::Rock => RPS::Scissors,
                RPS::Paper => RPS::Rock,
                RPS::Scissors => RPS::Paper,
            },
            Result::Draw => them,
            Result::Win => match them {
                RPS::Rock => RPS::Paper,
                RPS::Paper => RPS::Scissors,
                RPS::Scissors => RPS::Rock,
            },
        }
    }

    fn val(self) -> i32 {
        match self {
            Result::Lose => 0,
            Result::Draw => 3,
            Result::Win => 6,
        }
    }
}

fn score(their_shape: &str, result: &str) -> i32 {
    let them = RPS::from(their_shape);
    let result = Result::from(result);
    let me = result.shape(them);
    me.val() + result.val()
}

pub fn solution(input: &str) -> impl ToString {
    input
        .split("\n")
        .map(|line| {
            let mut state = line.split(" ");
            let their_shape = state.next().unwrap();
            let result = state.next().unwrap();
            score(their_shape, result)
        })
        .sum::<i32>()
}
