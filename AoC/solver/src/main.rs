use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let file = File::open("../day3/input.txt").expect("file wasn't found.");
    let reader = BufReader::new(file);

    let numbers: Vec<isize> = reader
        .lines()
        .map(|line| isize::from_str_radix(&line.unwrap().parse::<String>().unwrap(), 2).unwrap())
        .collect();

    let half_length = numbers.len() / 2;
    let n = 12;
    let mut count_array = [0; 12];
    let divider = 1 << (n-1);
    let complement = (1 << n) - 1;
    
    for number in numbers {
        for i in 0..n {
            count_array[i] += (number & ( divider >> i) != 0) as i32;
        }
    }
    let mut gamma = String::from("");
    for count in count_array {        
        if count < half_length as i32 {
            gamma.push('0');
        } else {
            gamma.push('1');
        }
    }

    let gamma_val = isize::from_str_radix(&gamma, 2).unwrap();
    println!("{}", gamma_val * (complement - gamma_val));
}
