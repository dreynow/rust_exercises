use std::io;

fn main() {
    println!("Welcome, enter what you want to do today");

    let mut todo = String::new();

    io::stdin().read_line(&mut todo).expect("Failed to read line");

    println!("Here is your todo: {}", todo);


}


