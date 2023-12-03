use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug)]
enum FSObject {
    File(i32),
    Dir(HashMap<String, FSObject>),
}

impl FSObject {
    fn parse(line: &str) -> (String, Self) {
        let (left, right) = line.split(" ").collect_tuple().unwrap();
        (
            right.into(),
            match left {
                "dir" => FSObject::Dir(HashMap::new()),
                a => FSObject::File(a.parse().unwrap()),
            },
        )
    }

    fn get_size(&self) -> i32 {
        match self {
            FSObject::File(size) => *size,
            FSObject::Dir(map) => map.iter().map(|(_, fs)| fs.get_size()).sum(),
        }
    }
}

#[derive(Debug)]
enum Command {
    LS,
    CD(String),
}

impl Command {
    fn parse(line: &str) -> Self {
        let mut tokens = line.split(" ").skip(1);
        let command = tokens.next().unwrap();
        match command {
            "cd" => Self::CD(tokens.next().unwrap().into()),
            "ls" => Self::LS,
            _ => panic!("Unknown command: {command}"),
        }
    }
}

#[derive(Debug, Clone)]
struct FileSystemError;

// TODO: turn into Result to debug this easier
fn traverse_filesystem<'fs>(
    fs: &'fs mut FSObject,
    path: &Vec<String>,
) -> Result<&'fs mut FSObject, FileSystemError> {
    match path.split_first() {
        Some((current, rest)) => match fs {
            FSObject::Dir(map) => match map.get_mut(current) {
                Some(fs_obj) => traverse_filesystem(fs_obj, &rest.to_vec()),
                None => Err(FileSystemError),
            },
            FSObject::File(_) => Err(FileSystemError),
        },
        None => Ok(fs),
    }
}

fn parse_filesystem(input: &str) -> FSObject {
    let mut lines = input.split("\n").peekable();
    let mut fs = FSObject::Dir(HashMap::from([("/".into(), FSObject::Dir(HashMap::new()))]));
    let mut current_path = Vec::<String>::new();

    while lines.peek().is_some() {
        let command = Command::parse(lines.next().unwrap());
        match command {
            Command::LS => match traverse_filesystem(&mut fs, &current_path) {
                Err(_) => panic!("Couldn't traverse to {current_path:?} on {fs:?}"),
                Ok(FSObject::File(_)) => panic!("Can't LS file!"),
                Ok(FSObject::Dir(map)) => lines
                    .peeking_take_while(|s| !s.starts_with("$"))
                    .map(FSObject::parse)
                    .for_each(|(dir_name, fs_obj)| {
                        map.insert(dir_name, fs_obj);
                    }),
            },
            Command::CD(d) if d == ".." => {
                current_path.pop();
            }
            Command::CD(dir_name) => {
                current_path.push(dir_name);
            }
        }
    }

    fs
}

fn get_all_directories<'f>(fs: &'f FSObject) -> Vec<&'f FSObject> {
    match fs {
        FSObject::File(_) => Vec::new(),
        FSObject::Dir(map) => {
            let mut v = map
                .iter()
                .flat_map(|(_, f)| get_all_directories(f))
                .collect::<Vec<_>>();
            v.push(fs);
            v
        }
    }
}

pub fn solution(input: &str) -> impl ToString {
    let fs = parse_filesystem(input);
    get_all_directories(&fs)
        .iter()
        .map(|f| f.get_size())
        .filter(|size| *size <= 100_000)
        .sum::<i32>()
}
