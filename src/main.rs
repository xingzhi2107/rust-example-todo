extern crate core;

mod model;

use model::Todo;

fn main() {
    Todo::complete_todo(3);
}
