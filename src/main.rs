use std::{io};

mod experiment;
mod days;

fn main() {

    let cmp_str: String = String::from("experiment");
    let mut path: String = String::new();

    io::stdin()
        .read_line(&mut path)
        .unwrap();
    
    remove_whitespace(&mut path);
    
    if cmp_str.eq(&path) {
        println!("\n\nexperiment  {} {}", path, cmp_str);
        experiment::experiment();
    } else {
        println!("\n\ndays  {} {}", path, cmp_str);
        days::day_one::day1_task1();
        days::day_one::day1_task2();
    }
}

fn remove_whitespace(s: &mut String) {
    s.retain(|c| !c.is_whitespace());
}