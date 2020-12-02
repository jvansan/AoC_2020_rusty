use std::env;
use std::fs;

#[derive(Debug)]
struct Line {
    min: usize,
    max: usize,
    letter: String,
    password: String,
}

fn line_to_struct(l: &str) -> Line {
    let c: Vec<String> = l.split_whitespace().map(|s| String::from(s)).collect();
    let range: Vec<usize> = c[0].split("-").map(|s| s.parse().unwrap()).collect();
    let letter = c[1].replace(":", "");
    return Line {
        min: range[0],
        max: range[1],
        letter: letter,
        password: c[2].clone(),
    };
}

fn test_line(l: &Line) -> bool {
    let c = l.password.matches(&l.letter).count();
    if c >= l.min && c <= l.max {
        return true;
    }
    return false;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("Reading file {}", filename);

    let content = fs::read_to_string(filename).expect("Something when wrong reading the file");
    let mut counter = 0;
    for c in content.split("\n") {
        let l = line_to_struct(c);
        if test_line(&l) {
            counter += 1;
            println!("{:?}", l)
        }
    }

    println!("{}", counter)
}
