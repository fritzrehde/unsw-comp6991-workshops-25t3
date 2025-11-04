// TODO:
// diff between function and closure
// closure is unique type that can't be written down
// unfortunately, e.g. FnOnce is all or nothing, can't make individual captured vars e.g. copy or &mut like in cpp
// explain move keyword
// recursive macro

use std::thread;

// Fn, FnMut, FnOnce

// implements Fn, FnMut, FnOnce
fn square(x: i32) -> i32 {
    x * x
}

// what about here?
fn square5(x: i32, v: &mut Vec<i32>) -> i32 {
    x * x
}

fn main() {
    let v = vec![1, 2, 3];

    // square1 implements Fn, FnMut, FnOnce
    let square1 = |x: i32| x * x;

    let mut s = String::from("hello world");

    // square2 implements FnMut, FnOnce
    let square2 = |x: i32| {
        s.push_str("from here");
        x * x
    };

    let square3 = |x: i32| {
        drop(s);
        x * x
    };

    my_map(v, square);
    // v.into_iter().map(|x| square(x)).collect::<Vec<_>>();

    let mut s = String::from("hello world");
    thread::spawn(|| {
        println!("hello world");
        drop(s);
    });

    let v = vec![1, 2, 3];
}

// different perspectives:
// - function/library author: best (least restrictive): Fn, worst: FnOnce
// - function user: best: FnOnce (can pass anything), worst: Fn

fn my_map<F>(v: Vec<i32>, f: F) -> Vec<i32>
where
    F: FnOnce(i32) -> i32,
{
    let mut output = vec![];
    for e in v {
        output.push(f(e));
    }
    output
}

fn mutate_each<F>(v: Vec<&mut i32>, mut f: F)
where
    F: Fn(&mut i32) -> i32,
{
    for e in v {
        f(e);
    }
}
