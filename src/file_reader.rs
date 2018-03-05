use std::{fs, path, io};
use std::io::BufRead;

pub fn read_file(path: &path::Path) -> io::Result<String> {
    fs::File::open(path).and_then(|file| {
        let mut buf_file = io::BufReader::new(file);

        let mut buffer = String::new();
        loop {
            match buf_file.read_line(&mut buffer) {
                Ok(0) => break, // EOF
                Ok(_) => continue,
                Err(e) => return Err(e),
            }
        }

        Ok(buffer)
    })
}