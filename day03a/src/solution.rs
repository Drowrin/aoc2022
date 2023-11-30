fn char_priority(c: char) -> i32 {
    let v = c as i32;

    if v > 90 {
        v - 96
    } else {
        v - 38
    }
}

pub fn solution(input: &str) -> String {
    input
        .split("\n")
        .map(|line| line.chars().map(char_priority).collect::<Vec<_>>())
        .map(|sack| {
            let size = sack.len() / 2;

            let (left, right) = sack.split_at(size);

            *left.iter().find(|item| right.contains(item)).unwrap()
        })
        .sum::<i32>()
        .to_string()
}
