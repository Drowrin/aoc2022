pub fn solution(input: &str) -> String {
    let mut elves = input
        .split("\n\n")
        .map(|elf| {
            elf.split("\n")
                .map(|item| item.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect::<Vec<i32>>();

    elves.sort();

    elves.into_iter().rev().take(3).sum::<i32>().to_string()
}
