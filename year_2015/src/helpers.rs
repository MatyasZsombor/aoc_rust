use std::fs;

pub fn read_into_string(path: &str) -> String
{
    fs::read_to_string(path).expect("Can't find file")
}