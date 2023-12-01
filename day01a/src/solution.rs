pub fn solution(input: &str) -> impl ToString {
    input
        .split("\n\n")
        .map(|elf| {
            elf.split("\n")
                .map(|item| item.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .max()
        .expect("Empty input!")
}
