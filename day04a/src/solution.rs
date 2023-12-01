use itertools::Itertools;

fn parse_elf(elf: &str) -> (i32, i32) {
    elf.split("-")
        .map(str::parse)
        .map(Result::unwrap)
        .collect_tuple()
        .unwrap()
}

fn parse_line(line: &str) -> ((i32, i32), (i32, i32)) {
    line.split(",").map(parse_elf).collect_tuple().unwrap()
}

fn check_overlap((left, right): &((i32, i32), (i32, i32))) -> bool {
    (left.0 <= right.0 && left.1 >= right.1) || (left.0 >= right.0 && left.1 <= right.1)
}

pub fn solution(input: &str) -> impl ToString {
    input
        .split("\n")
        .map(parse_line)
        .filter(check_overlap)
        .count()
}
