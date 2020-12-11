use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

pub fn read_file(file_name: &str) -> Result<Vec<String>, std::io::Error> {
    let path = Path::new(file_name);
    let mut file = File::open(&path)?;
    io::BufReader::new(file).lines().collect()
}
