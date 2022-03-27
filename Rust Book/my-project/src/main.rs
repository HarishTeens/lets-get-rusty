use rand::Rng;
use std::{io, io::Write, collections::HashMap};
use std::collections::*;

fn main() {
    let v:Vec<isize> = Vec::new();
    let mut v = vec![1, 2, 3];

    v.push(4);
    v.push(5);
    v.push(6);
    let third: i32 =  v[2];
    v.push(6);
    // third = 33;
    println!("The third element is {}", third);

    match v.get(100) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element")
    };

    let mut v = vec![100, 32, 57];
    for i in &mut v {   
        *i += 50;     
        println!("{}", i);
    }

    enum SpreadSheetCell {
        Int(i32),
        Text(String),
        Float(f64)
    }

    let row = vec![
        SpreadSheetCell::Int(2),
        SpreadSheetCell::Float(1.1)
    ];

}