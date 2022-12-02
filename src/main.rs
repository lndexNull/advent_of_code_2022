use std::io;

mod experiment;
mod days;

fn main() {

    let cmp_str: String = String::from("experiment");
    let mut path: String = String::new();

    io::stdin()
        .read_line(&mut path)
        .unwrap();
    

    if cmp_str == path { //TODO figure out how to compare strings properly
        println!("experiment  {}    {}", path, cmp_str);
        experiment::experiment();
    } else {
        println!("days  {}    {}", path, cmp_str);
        days::day_one::day1_task1();
        days::day_one::day1_task2();
    }
}
