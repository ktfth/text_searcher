use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 3 {
        let path = Path::new(&args[1]);
        let display = path.display();

        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };

        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("couldn't read {}: {}", display, why),
            Ok(_) => {
                print!("{} contains:\n{}", display, s);
                if s.contains(&args[2]) {
                    print!("{} have: {}", display, args[2]);
                }
            },
        }
    }
}
