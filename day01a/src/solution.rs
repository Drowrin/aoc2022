pub fn solution(input: &str) -> String {
    input
        .split("\n\n")
        .map(|elf| {
            elf.split("\n")
                .map(|item| item.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .max()
        .expect("Empty input!")
        .to_string()
}
