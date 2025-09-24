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

#[derive(Debug, Clone)]
struct Test {
    arr: [i32; 3],
    a: i32,
}

#[derive(Clone)]
struct Test2 {
    vec: Vec<i32>,
    a: i32,
}

fn count(s: &str) -> HashMap<char, usize> {
    let mut map = HashMap::new();
    for c in s.chars() {
        // map.entry(c).and_modify(|count| *count += 1).or_insert(1);h
        let count = map.entry(c).or_insert(0);
        *count += 1;

        // Python:
        // map = defaultdict(lambda: 0)
        // for c in s:
        //   map[c] += 1
    }
    for (k, v) in map.iter() {}
    map
}

fn main() {
    dbg!(count("hello world"));
    // String
    // &str
    // let string_slice: &str = "this has fixed length, can't grow";
    // let mut s = String::from("this literal has fixed length, but i'm converting it to a String");
    // s.push('a');
    // let string_slice_ref_to_String: &str = s.as_str();
    // let string_slice_ref_to_String: &String = &s;
    // take_str_ref(&s);
    // take_str_ref(string_slice);

    // let v1 = vec![1, 2, 3];
    // let v2 = v1;
    // dbg!(v1);
    // let test2 = Test {
    //     arr: [1, 2, 3],
    //     a: 10,
    // };
    // let test2_2 = test2.clone();

    // let v1 = 10;
    // let v2 = v1;
    // dbg!(v1);

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
