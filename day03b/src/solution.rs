fn char_priority(c: char) -> i32 {
    let v = c as i32;

    if v > 90 {
        v - 96
    } else {
        v - 38
    }
}

pub fn solution(input: &str) -> impl ToString {
    input
        .split("\n")
        .map(|line| line.chars().map(char_priority).collect::<Vec<_>>())
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|group| {
            let mut elves = group.iter();

            let first = elves.next().unwrap();
            let second = elves.next().unwrap();
            let third = elves.next().unwrap();

            *first
                .iter()
                .find(|item| second.contains(item) && third.contains(item))
                .unwrap()
        })
        .sum::<i32>()
}
