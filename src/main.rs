use std::env;
use std::fs::File;
use std::{fs, io};
use std::io::prelude::*;
use std::path::Path;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let mut entries = fs::read_dir(".")?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>()?;
        entries.sort();
        for entry in entries {
            if !entry.is_dir() {
                let path = Path::new(&entry);
                let display = path.display();

                let mut file = match File::open(&path) {
                    Err(why) => panic!("couldn't open {}: {}", display, why),
                    Ok(file) => file,
                };

                let mut s = String::new();
                match file.read_to_string(&mut s) {
                    Err(why) => panic!("couldn't read {}: {}", display, why),
                    Ok(_) => {
                        if s.contains(&args[1]) {
                            print!("{} contains:\n{}", display, s);
                            print!("{} have: {}", display, args[1]);
                        }
                    },
                }
            }
        }
    }

    Ok(())
}
