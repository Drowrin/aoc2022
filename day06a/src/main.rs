mod solution;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        let examples = include_str!("../data/example.txt").split("\n");
        let answers = include_str!("../data/answer.txt")
            .split("\n")
            .collect::<Vec<_>>();
        assert_eq!(
            examples
                .map(solution::solution)
                .map(|x| x.to_string())
                .collect::<Vec<_>>(),
            answers
        );
    }
}

fn main() {
    let start = std::time::Instant::now();
    let answer = solution::solution(include_str!("../data/input.txt"));
    let duration = start.elapsed();

    println!("Done in {:?}", duration);
    print!("{}", answer.to_string());
}
