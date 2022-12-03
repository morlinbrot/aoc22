use std::fs;

pub fn read_split<'a>(path: &str) -> Vec<String> {
    let str: String = fs::read_to_string(path).expect("Failed to read file");
    str.split("\n").map(Into::into).collect()
}
