fn has_repeat(bytes: &&[u8]) -> bool {
    for i in 0..bytes.len() {
        for j in 0..bytes.len() {
            if j != i && bytes[i] == bytes[j] {
                return true;
            }
        }
    }
    return false;
}

pub fn solution(input: &str) -> impl ToString {
    input.as_bytes().windows(14).take_while(has_repeat).count() + 14
}
