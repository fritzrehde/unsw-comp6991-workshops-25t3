// Ownership rules:
// - Each value in Rust has an owner.
// - There can only be one owner at a time.
// - When the owner goes out of scope, the value will be dropped.

fn main() {
    println!("Hello, world!");
    let x = [1, 2, 3]
        .into_iter()
        .map(|x| 5 * x)
        .filter(|x| x % 2 == 0)
        .collect::<Vec<u32>>();
}
