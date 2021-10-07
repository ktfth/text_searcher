use std::env;
use regex::Regex;
use std::{fs, io};
use std::io::Read;
use std::fs::File;
use regex::RegexSet;
use std::path::Path;
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

fn search(entry: String) {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&entry);
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(_) => panic!(""),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(_) => print!(""),
        Ok(_) => {
            let re = Regex::new(&args[1]).unwrap();
            if re.is_match(&s) {
                let split = s.split("\n");
                let lines = split.collect::<Vec<&str>>();
                println!("{}:", display);
                for i in 0..lines.len() {
                    if lines[i] != "" && re.is_match(&lines[i]) {
                        println!("{}| {}", i + 1, lines[i]);
                    }
                }
            }
        }
    }
}

fn search_content(raw_path: String) {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        search(raw_path.clone());
    } else if args.len() > 2 {
        if args.iter().position(|r| r == "--include") != None && args.iter().position(|r| r == "--ignore") != None {
            let include_index = args.iter().position(|r| r == "--include").unwrap();
            let ignore_index = args.iter().position(|r| r == "--ignore").unwrap();
            if include_index < ignore_index {
                let include_set = RegexSet::new(&args[include_index + 1..ignore_index]).unwrap();
                let include_matches: Vec<_> = include_set.matches(&raw_path).into_iter().collect();
                let ignore_set = RegexSet::new(&args[ignore_index + 1..]).unwrap();
                let ignore_matches: Vec<_> = ignore_set.matches(&raw_path).into_iter().collect();
                let cloned = raw_path.clone();

                if include_matches.len() > 0 && ignore_matches.len() == 0 {
                    search(cloned);
                }
            } else if ignore_index < include_index {
                let ignore_set = RegexSet::new(&args[ignore_index + 1..include_index]).unwrap();
                let ignore_matches: Vec<_> = ignore_set.matches(&raw_path).into_iter().collect();
                let include_set = RegexSet::new(&args[include_index + 1..]).unwrap();
                let include_matches: Vec<_> = include_set.matches(&raw_path).into_iter().collect();
                let cloned = raw_path.clone();

                if include_matches.len() > 0 && ignore_matches.len() == 0 {
                    search(cloned);
                }
            }
        } else if args.iter().position(|r| r == "--include") != None {
            let include_index = args.iter().position(|r| r == "--include").unwrap();
            let include_set = RegexSet::new(&args[include_index + 1..]).unwrap();
            let include_matches: Vec<_> = include_set.matches(&raw_path).into_iter().collect();
            let cloned = raw_path.clone();

            if include_matches.len() > 0 {
                search(cloned);
            }
        } else if args.iter().position(|r| r == "--ignore") != None {
            let ignore_index = args.iter().position(|r| r == "--ignore").unwrap();
            let ignore_set = RegexSet::new(&args[ignore_index + 1..]).unwrap();
            let ignore_matches: Vec<_> = ignore_set.matches(&raw_path).into_iter().collect();
            let cloned = raw_path.clone();

            if ignore_matches.len() == 0 {
                search(cloned);
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        let current_dir = Path::new(".");
        let paths = collect_file_from_dir(current_dir);
        for path in paths {
            for entry in path {
                let entry_path = entry.to_string_lossy().to_string();
                search_content(entry_path);
            }
        }
    } else {
        println!("text-search [pattern-or-regexp-in-file] [--ignore|--include] [pattern-or-regexp-in-path]");
    }
}
