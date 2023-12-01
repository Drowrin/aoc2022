use itertools::Itertools;

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

fn find_badge(group: &[Vec<i32>]) -> i32 {
    let (first, second, third) = group.iter().collect_tuple().unwrap();

    *first
        .iter()
        .find(|item| second.contains(item) && third.contains(item))
        .unwrap()
}

pub fn solution(input: &str) -> impl ToString {
    input
        .split("\n")
        .map(parse_line)
        .collect::<Vec<_>>()
        .chunks(3)
        .map(find_badge)
        .sum::<i32>()
}
