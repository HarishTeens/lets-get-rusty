use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let file = File::open("../day1/input.txt").expect("file wasn't found.");
    let reader = BufReader::new(file);

    let numbers: Vec<i64> = reader
        .lines()
        .map(|line| line.unwrap().parse::<i64>().unwrap())
        .collect();

    let mut increment = 0;
    if numbers.len() <= 3 {return;}
    let mut window = numbers[0] + numbers[1] + numbers[2];
    let mut idx = 3;
    while idx < numbers.len() {        
        if window + numbers[idx] - numbers[idx-3]  > window {
            increment += 1;
        }
        window = window + numbers[idx] - numbers[idx-3];
        idx += 1;
    }

    println!("{}",increment);
}
