// to cover:

// all builds stored in one dir (easy to delete): `export CARGO_TARGET_DIR="$HOME/cargo-targets"`

// ownership rules:
// - Each value in Rust has **one** owner.
// - There can only be one owner at a time.
// - When the owner goes out of scope, the value will be dropped.
// use after free, double free, dangling pointers (next week), thread safety

// immutable vs mutable

// how can we simulate referencing objects temporarily (both immut and mut) without proper rust references?

// structs, code blocks, enums, pattern matching, errors as values, unwrapping

// () unit type (and value), demo match returning () or {}

// if, match are expressions (return value)

// ranges

// &str vs String

enum Colour {
    Red,
    Green,
    Blue,
}

enum OptionState {
    Some,
    None,
}

// struct Option {
//     value_if_some: *String,
//     state: OptionState,
// }todo!()

// "illegal": Option { "String", state: None}, Option {null, state: Some}

// enum Option<T> {
//     Some(T),
//     None,
// }

fn unwrap<T>(opt: Option<T>) -> T {
    match opt {
        Option::Some(s) => s,
        Option::None => panic!("sorry, value wasn't here"),
    }
}

enum Result {
    Ok(String),
    Err(String),
}

fn attempt_to_do_sth() -> Option<String> {
    match read_from_env("path1") {
        Some(path1_val) => match read_from_env("path2") {
            Some(path2_val) => todo!(),
            None => None,
        },
        None => None,
    }
}

fn attempt_to_do_sth2() -> Option<String> {
    let path1_val = read_from_env("path1")?;
    let path2_val = read_from_env("path2")?;
    todo!()
}

fn read_from_env(path: &str) -> Option<String> {
    todo!()
}

fn main() {
    // panic!("not yet implemented");
    let s1 = String::from("hello world");
    // dbg!(&s1);
    let (s1, len) = consume_string(s1.clone());
    // dbg!(s1);
    // s1.push('a');

    // let mut s2 = String::from("hello world");
    // s2.push('a');
    let b = {
        let s3 = String::from("hi");
        s3
    };
    // dbg!(b);

    let (a, b): (i32, i32) = (10, 20);

    let val = if 10 == 11 {
        String::from("hello")
    } else {
        String::from("world")
    };

    let arr = vec![1, 2, 3];
    // dbg!(arr[3]);
    // dbg!(arr.get(3).unwrap());
    dbg!(
        arr.get(3)
            .expect("some justification as to why this will never be called")
    );
    match arr.get(2) {
        Some(s) => {
            dbg!(s);
        }
        None => {
            println!("there was nothing here");
        }
    }

    println!("program continue");
}

struct RetVal {
    returned_string: String,
    string_len: usize,
}

fn consume_string(mut s: String) -> (usize, String) {
    s.push('a');
    // println!("{}", &s);
    // implicit
    // s.drop()
    let (len, s_moved) = (s.len(), s);

    (len, s_moved)
    // return (len, s_moved);
}
