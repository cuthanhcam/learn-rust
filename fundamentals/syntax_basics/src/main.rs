mod control_flow;
mod data_types;
mod functions;
mod variables;

fn main() {
    println!("== Syntax Basics ==");
    variables::run();
    data_types::run();
    functions::run();
    control_flow::run();
}
