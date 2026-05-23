mod enums;
mod if_let;
mod match_control_flow;

fn main() {
    println!("=== Enums and Pattern Matching ===");
    enums::run();
    match_control_flow::run();
    if_let::run();
}
