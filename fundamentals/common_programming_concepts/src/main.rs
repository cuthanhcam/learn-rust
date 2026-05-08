mod comments;
mod control_flow;
mod data_types;
mod functions;
mod variables_mutability;

fn main() {
    println!("== Common Programming Concepts in Rust ==");

    variables_mutability::run();
    data_types::run();
    functions::run();
    comments::run();
    control_flow::run();
}
