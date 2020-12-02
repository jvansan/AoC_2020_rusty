use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("Reading file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something when wrong reading the file");

    let numbers: Vec<i32> = contents.split('\n').map(|s| s.parse().unwrap()).collect();

    let mut result = 0;
    for n in numbers.iter() {
        for m in numbers.iter() {
            if n + m == 2020 {
                result = n * m
            }
        }
    }
    println!("{}", result);
}
