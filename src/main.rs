use std::env;
use std::{fs, io};
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::path::PathBuf;

fn collect_file_from_dir(dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut result: Vec<PathBuf> = vec![];
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                result.extend(collect_file_from_dir(&path)?);
            } else {
                result.push(path);
            }
        }
    }
    Ok(result)
}

fn search_content(entry: String) {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&entry);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(_) => print!(""),
        Ok(_) => {
            if s.contains(&args[1]) {
                print!("{} contains:\n{}", display, s);
                print!("{} have: {}", display, args[1]);
            }
        }
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let current_dir = Path::new(".");
        let paths = collect_file_from_dir(current_dir);
        for path in paths {
            for entry in path {
                let entry_path = entry.to_string_lossy().to_string();
                search_content(entry_path);
            }
        }
    }

    Ok(())
}
