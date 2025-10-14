// Ownership rules:
// - Each value in Rust has an owner.
// - There can only be one owner at a time.
// - When the owner goes out of scope, the value will be dropped.

// forget free: memory leak
// free, and still use (use after free): UB
// double free: UB

// manual memory management:
// - C, Asm, Zig

// garbage collection:
// - does the freeing for you

// Swift:
// - reference counting

// HashSet<T> = HashMap<T, ()>

use std::collections::{HashMap, LinkedList, VecDeque};

#[derive(Debug, Clone)]
struct Foo {
    i: i32,
}

impl Copy for Foo {}

fn foo2(mut s: String) -> String {
    dbg!(&s);
    s.push('c');
    s
}

fn foo2_proper(s: &String) {
    dbg!(&s);
}

fn foo2_proper_mut(s: &mut String) {
    dbg!(&s);
    s.push('c');
}

fn foo3(s: u32) {
    dbg!(s);
}

// Copy, Clone
// Debug

fn foo() -> i32 {
    let mut s = String::from("hello");
    let s2 = s.clone();
    // s = foo2(s);
    // foo2_proper(&s);
    foo2_proper_mut(&mut s);
    dbg!(s);

    let s = 10;
    // let s2 = s; // bitwise/shallow copy, because u32 impls Copy trait
    // s.copy();
    // s.clone();
    foo3(s);
    dbg!(s);

    // let x: u32 = 10;
    let z = {
        let x = 10;
        // private computation.
        let y = 10;
        x
    };

    let x = 10;

    // let mut z = String::from("default");
    // if x > 10 {
    //     z = String::from("hello");
    // } else {
    //     // z = String::from("world");
    // };

    // let z = if 11 > 10 {
    //     String::from("hello")
    // } else if x >= 10 {
    //     String::from("world")
    // } else {
    //     String::from("2")
    // };
    // dbg!(z);

    // dbg!(y);
    // dbg!(x);
    // x.drop();

    // return z;
    z
}

// fn fallible_fn(param1: u32, param2: f64, ret_val: **T) -> i32 {
// // computation
// if .. {
//   return -1;
// } else {
//   ret_val = malloc();
//   return 0;
// }
// }

type T = u32;

// union C

// enum Option {
//     Some(u64),
//     None,
// }

enum Node {
    Next(Box<Node>),
    None,
}

enum CustomError {
    ConnectionFailed { err: String, err2: String },
    InvalidQuery((String, String)),
    BadRequest,
}

enum Error {
    Ok(T),
    Err(CustomError),
}

fn fallible_fn(param1: u32, param2: f64) -> Option<u32> {
    if true { Option::None } else { Option::Some(10) }
}

fn complex_fn() -> Option<f64> {
    let x = fallible_fn(10, 10.0)?;
    let y = fallible_fn(20, 20.0)?;
    let z = fallible_fn(30, 30.0)?;
    Some((x + y + z) as f64)
}

fn complex_fn2() -> Option<f64> {
    match fallible_fn(10, 10.0) {
        Some(x) => match fallible_fn(20, 20.0) {
            Some(y) => match fallible_fn(30, 30.0) {
                Some(z) => Some((x + y + z) as f64),
                None => return None,
            },
            None => return None,
        },
        None => return None,
    }
}

fn debug_things<T: std::fmt::Debug>(v: Vec<T>) {
    for e in v {
        dbg!(e);
    }
}

fn debug_things_u32(v: Vec<u32>) {
    for e in v {
        dbg!(e);
    }
}

fn debug_things_i32(v: Vec<i32>) {
    for e in v {
        dbg!(e);
    }
}

fn main() {
    debug_things::<u32>(vec![1, 2, 3]);
    debug_things::<i32>(vec![1, 2, 3]);
    debug_things::<u64>(vec![1, 2, 3]);

    let z = foo();

    // Box<dyn Trait>

    let x = match fallible_fn(10, 10.0) {
        Option::Some(t) => {
            dbg!(t);
            20
        }
        Option::None => 10,
    };

    let v = vec![1, 2, 3];
    // for e in &v {
    //     // &v lifetime starts here
    //     v.push(4); // &mut starts and ends here
    // }
    // // &v lifetime ends here

    // entry api for hashmaps

    // python
    // d = defaultdict(lambda: 0)
    // for e in v:
    //   d[e] += 1
    //   d.get(e, default=0) += 1

    let mut d = HashMap::new();
    for e in &v {
        *d.entry(e).or_insert_with(|| 0) += 1;
    }


    Mutex<T> mutex = Mutex::new(Vec::new());
    // Mutex<T>::lock(&self) -> &mut T
    thread::spawn(
        // &mut ->
        || {
        loop {
            let mut_ref = mutex.lock();
            // do mutating stuff  with mut_ref
        }
    });
    // guarantee this thread ends before the next called
    thread::spawn(
        // &mut ->
        || {
        loop {
            let mut_ref = mutex.lock();
            // do mutating stuff  with mut_ref
        }
    });

    // let s = v.size()
    // for (int i = 0; s; ++i) {
    // }

    // let b = Box::new(10); // malloc

    // let d = LinkedList::new();

    println!("Hello, world!");
    let x = [1, 2, 3]
        .into_iter()
        .map(|x| 5 * x)
        .filter(|x| x % 2 == 0)
        .collect::<Vec<u32>>();

    // z.drop/z.free/z.destructor
}

// let ret = Vec::new();
// for e in v:
//     let y = map(x):
//     if (y % 2 != 0) {
//         continue;
//     }
//     ret.push_back(y)
