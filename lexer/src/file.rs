use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn get_file_content(file_path: &str) -> Vec<char> {
    let work_dir = env::current_dir().unwrap();
    let full_file_path = work_dir.join(Path::new(file_path));
    
    let mut file = File::open(full_file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    contents.chars().collect()
}