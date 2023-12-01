fn get_elf_calories(elf: &str) -> i32 {
    elf.split("\n")
        .map(str::parse::<i32>)
        .map(Result::unwrap)
        .sum()
}

pub fn solution(input: &str) -> impl ToString {
    let mut elves = input
        .split("\n\n")
        .map(get_elf_calories)
        .collect::<Vec<i32>>();

    elves.sort();

    elves.into_iter().rev().take(3).sum::<i32>()
}
