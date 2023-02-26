extern crate core;

use std::io::Write;

mod model;
mod command;

fn main() {
    println!("Welcome to todo list! Use 'help' command to show help info!");
    let mut todo_app = model::TodoApp::new();
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");
        command::deal_input(&mut todo_app, &input);
    }
}
