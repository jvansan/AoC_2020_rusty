use std::env;
use std::fs;

#[derive(Debug)]
struct Line {
    p1: usize,
    p2: usize,
    letter: String,
    password: String,
}

fn line_to_struct(l: &str) -> Line {
    let c: Vec<String> = l.split_whitespace().map(|s| String::from(s)).collect();
    let range: Vec<usize> = c[0]
        .split("-")
        .map(|s| s.parse::<usize>().unwrap())
        .map(|s| s - 1)
        .collect();
    let letter = c[1].replace(":", "");
    return Line {
        p1: range[0],
        p2: range[1],
        letter: letter,
        password: c[2].clone(),
    };
}

fn get_nth_char(s: &String, i: usize) -> String {
    return String::from(s.as_bytes()[i] as char);
}

fn test_line(l: &Line) -> bool {
    if get_nth_char(&l.password, l.p1) == l.letter {
        if get_nth_char(&l.password, l.p2) == l.letter {
            return false;
        }
        return true;
    } else {
        if get_nth_char(&l.password, l.p2) == l.letter {
            return true;
        }
        return false;
    }
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
            println!("{:?}", l);
        }
    }

    println!("{}", counter);
}
