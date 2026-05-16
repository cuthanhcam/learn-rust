mod borrowing;
mod ownership;
mod slice_type;

fn main() {
    println!("== Understanding Ownership in Rust ==");

    ownership::run();
    borrowing::run();
    slice_type::run();
}
