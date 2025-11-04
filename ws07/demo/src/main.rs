// TODO:
// diff between function and closure
// closure is unique type that can't be written down
// unfortunately, e.g. FnOnce is all or nothing, can't make individual captured vars e.g. copy or &mut like in cpp
// explain move keyword
// recursive macro

use std::{sync::Arc, thread};

// fn: function pointer
// Fn, FnMut, FnOnce

fn run_twice(mut f: impl FnMut(&mut String), payload: &mut String) {
    f(payload);
    f(payload);
}

// implements all of them
fn square(x: i32) -> i32 {
    x * x
}

// implements ?
fn square5(x: i32, v: &mut Vec<i32>) -> i32 {
    x * x
}

fn main() {
    // let square = |x| {
    //     println!("string: {}", s);
    //     x * x
    // };
    // run_twice(square);
    square(10);

    let mut s = String::from("hello world");
    let mut x: i32 = 10;
    let foo1 = |s: &mut String| {
        s.push('a');
    };
    run_twice(foo1, &mut s);

    let foo2 = || {
        println!("{}", s);
    };

    let foo3 = || {
        s.push('a');
    };

    let foo4 = || {
        println!("{}", s);
        x += 1;
    };

    let y = String::from("hello world 2");

    // struct ClosureType {
    //     fn_ptr: fn(),
    //     // captured variables
    //     String &s,
    //     String d,
    // };

    // fn ClosureType::call(&self) {
    //     println!("{}", &self.s);
    //     // drop(y);
    // }

    let foo4/*ClosureType*/ = move || {
        println!("{}", s);
        println!("{}", s);
        // drop(y);
    };
    dbg!(&s);
    foo4();
    foo4();
    // foo4();

    let shared_state: Arc<String> = Arc::new(String::from("hello world"));
    for _ in 0..10 {
        // &Arc -> Arc using clone()
        let shared_state_for_thread = Arc::clone(&shared_state);
        std::thread::spawn(move || {
            // drop(s);
            dbg!(&shared_state_for_thread);
            let _ = shared_state_for_thread.len();
            // shared_state_for_thread.push('c');
            drop(shared_state_for_thread)
        });
    }

    let shared_state: Arc<String> = Arc::new(String::from("hello world"));
    // reference counting: Arc<String>.clone()
    let shared_state_for_thread1 = Arc::clone(&shared_state);
    let shared_state_for_thread2 = Arc::clone(&shared_state);
    std::thread::spawn(move || {
        // drop(s);
        let shared_state_for_thread3 = Arc::clone(&shared_state_for_thread1);
        std::thread::spawn(move || loop {
            dbg!(&shared_state_for_thread3);
        });
        loop {
            dbg!(&shared_state_for_thread1);
        }
        //
    });
    std::thread::spawn(move || {
        // drop(s);
        dbg!(&shared_state_for_thread2);
        drop(shared_state_for_thread2)
    });

    //

    let v = vec![1, 2, 3];

    // square1 implements ?
    let square1 = |x: i32| x * x;

    let mut s = String::from("hello world");

    // square2 implements ?
    let square2 = |x: i32| {
        s.push_str("from here");
        x * x
    };

    // square3 implements FnOnce
    // let square3 = |x: i32| {
    //     drop(s);
    //     x * x
    // };

    let square4 = || {
        println!("{}", s);
        let x = 10;
        x * x;
        drop(s);
    };
    // dbg!(&s);
    square4();
    // square4();
    // thread::spawn(square4);
    // thread::spawn(square4);

    // my_map(v, square);
    // v.into_iter().map(|x| square(x)).collect::<Vec<_>>();

    let mut s = String::from("hello world");
    // thread::spawn(|| {
    //     println!("hello world");
    //     drop(s);
    // });

    let fn_mut = |x: i32| -> i32 {
        s.push(' ');
        10
    };
    mutate_each2(v, fn_mut);
}

// different perspectives:
// - function/library author: best (least restrictive): Fn, worst: FnOnce
// - function user: best: FnOnce (can pass anything), worst: Fn

fn foo<F>(mut f: F)
where
    F: FnMut() -> i32,
{
    for e in [1, 2, 3] {
        f();
    }
}

fn main2() {
    let mut s = String::from("hello world");
    let closure1 = || {
        dbg!(&s);
        10i32
    };
    foo(closure1);
    let closure2 = || {
        s.push('a');
        10i32
    };
    foo(closure2);
    let closure3 = || {
        drop(s);
        10i32
    };
    // foo(closure3);
}

// fn my_map<F>(v: Vec<i32>, f: F) -> Vec<i32>
// where
//     F: FnOnce(i32) -> i32,
// {
//     let mut output = vec![];
//     for e in v {
//         output.push(f(e));
//     }
//     output
// }

fn mutate_each2<F>(v: Vec<i32>, mut f: F)
where
    F: FnMut(i32) -> i32,
{
    for e in v {
        f(e);
    }
}

fn mutate_each<F>(v: Vec<&mut i32>, f: F)
where
    F: Fn(&mut i32) -> i32,
{
    for e in v {
        f(e);
    }
}
