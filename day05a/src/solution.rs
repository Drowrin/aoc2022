use itertools::Itertools;

struct Step {
    m: usize,
    f: usize,
    t: usize,
}

fn parse_arrangement(lines: &str) -> Vec<Vec<char>> {
    let data = lines
        .split("\n")
        .map(|line| line.chars().skip(1).step_by(4).collect_vec())
        .collect_vec();

    (0..data.get(0).unwrap().len())
        .map(|i| {
            data.clone()
                .iter()
                .rev()
                .skip(1) // line numbers
                .map(|d| d.clone()[i])
                .filter(|s| *s != ' ')
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn parse_procedure(data: &str) -> Vec<Step> {
    data.split("\n")
        .map(|line| {
            line.split(" ")
                .skip(1)
                .step_by(2)
                .map(str::parse)
                .map(Result::unwrap)
                .collect_tuple()
                .unwrap()
        })
        .map(|(m, f, t)| Step { m, f, t })
        .collect()
}

pub fn solution(input: &str) -> impl ToString {
    let (arrangement_data, procedure_data) = input.split("\n\n").collect_tuple().unwrap();
    let mut arrangement = parse_arrangement(arrangement_data);
    let procedure = parse_procedure(procedure_data);

    for step in procedure {
        let split_index = arrangement[step.f - 1].len() - step.m;
        let v = arrangement[step.f - 1].split_off(split_index);
        arrangement[step.t - 1].extend(v.iter().rev());
    }

    arrangement
        .iter()
        .map(|column| column.last().unwrap())
        .collect::<String>()
}
