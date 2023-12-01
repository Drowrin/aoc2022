fn get_elf_calories(elf: &str) -> i32 {
    elf.split("\n")
        .map(str::parse::<i32>)
        .map(Result::unwrap)
        .sum()
}

pub fn solution(input: &str) -> impl ToString {
    input
        .split("\n\n")
        .map(get_elf_calories)
        .max()
        .expect("Empty input!")
}
