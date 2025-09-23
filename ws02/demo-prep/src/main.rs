// ownership rules:
// - Each value in Rust has **one** owner.
// - There can only be one owner at a time.
// - When the owner goes out of scope, the value will be dropped.
// use after free, double free, dangling pointers, thread safety, iterator invalidation

// stack vs heap, Copy vs Clone trait
// array vs vector
// What is the default when copying? (deep vs shallow)
// &str vs String
// moving, &, &mut, rules

// Show fancy iterator stuff (map, filter, collect, fold).
// entry API: python's defaultdict in rust (but better), demo how to build char freq map

use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
struct Test {
    arr: [i32; 100000],
    a: i32,
}
// impl Copy for Test {}

fn main() {
    // let mut hashmap: HashMap<&str, i32> = HashMap::new();
    // hashmap.insert("test", 10);
    // hashmap.insert("another", 20);

    // let mut array = [1, 2, 3];
    // let array = [0; 1000];
    // array.push(5);

    // let vector_ref = &mut vector; // vector_ref lifetime starts here
    // println!("{}", vector.len()); // <unnamed ref> lifetime starts and ends here.
    // push_element_to_vec(vector_ref); // vector_ref lifetime ends here
}

fn push_element_to_vec(v: &Vec<i32>) {
    // I need &mut
    // v.push(5)
    // End of scope: every owned value gets dropped.
}
