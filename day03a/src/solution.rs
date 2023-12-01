fn char_priority(c: char) -> i32 {
    let v = c as i32;

    if v > 90 {
        v - 96
    } else {
        v - 38
    }
}

fn parse_line(line: &str) -> Vec<i32> {
    line.chars().map(char_priority).collect::<Vec<_>>()
}

fn find_misplaced(sack: Vec<i32>) -> i32 {
    let (left, right) = sack.split_at(sack.len() / 2);
    *left.iter().find(|item| right.contains(item)).unwrap()
}

pub fn solution(input: &str) -> impl ToString {
    input
        .split("\n")
        .map(parse_line)
        .map(find_misplaced)
        .sum::<i32>()
}
