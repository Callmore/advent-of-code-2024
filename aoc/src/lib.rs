use std::{fs::File, io};

fn return_file_content(name: &str) -> Result<String, io::Error> {
    let input = File::open(name)?;
    Ok(io::read_to_string(input)?)
}

pub fn load_input() -> Result<String, io::Error> {
    return_file_content("../input.txt")
}

pub fn load_example_input() -> Result<String, io::Error> {
    return_file_content("../example_input.txt")
}

pub fn load_input_custom_name(name: &str) -> Result<String, io::Error> {
    return_file_content(format!("../{}", name).as_str())
}
