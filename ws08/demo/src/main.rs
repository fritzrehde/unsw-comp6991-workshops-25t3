// TODO:
// concurrency (dealing with multiple things at once) vs parallelism (doing multiple things at once)
// send, sync traits (compare to how Copy is a building block every uses/depends on given simple rules): !send,!sync=Rc, send,!sync=Cell, !send,sync=MutexGuard
// thread::spawn vs thread::scope
// demo how two &mut doesn't compile
// Why do we need Arc (why not just Rc)?
// sharing state across threads: Arc<Mutex> (try Rc mutex, explain why doesn't work), rwlock

// common mistakes with mutex:
// 1. always need to unlock, otherwise deadlock (rust fix: mutex gets released/unlocked automatically at end of scope)
// 2. use mutex, but don't actually lock and unlock, so race condition still exists (rust fix: you don't even get mutable access without the mutex)

// channels: do not communicate by sharing memory, share memory by communicating (shared memory synchronisation vs message passing)
// demo we can still deadlock
// global mutable state: LazyLock

// send + sync: most things (safe to be sent between threads, shared between threads)
// !send + !sync: types that are not thread-safe (e.g. Rc), not safe to send, not safe to share
// send + !sync: e.g. Cell, move between threads, but can't be shared (because interior mutability)
// !send + sync: e.g. MutexGuard, rule in linux (pthread mutex), if you lock on thread A, thread A needs to unlock it

use std::{
    cell::Cell,
    rc::Rc,
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
};

// global mutable state is UNSAFE!!! but i want it anyways
static mut i: i32 = 10;

// global heap-allocated state
// static s: String = String::from("hello");

fn main() {
    println!("Hello, world!");

    // i = 10;

    // let cell = Cell::new(10);
    // let cell_ref1 = &cell;
    // let cell_ref2 = &cell;
    // thread::spawn(move || loop {
    //     cell_ref1.set(5);
    // });
    // thread::spawn(move || loop {
    //     cell_ref2.set(5);
    // });

    // let rc = Rc::clone(1);
    // let rc_clone = Rc::clone(&rc);
    // // why is Rc not Send-safe: spawn two threads, move each into one thread, and then clone infinitely many times.

    let mut s1 = String::from("hello world");
    // let f = || {
    //     // dbg!(&s);
    //     s.push('a');
    //     // drop(s);
    // };
    // thread::scope(|s| {
    //     // let a = 10;
    //     s.spawn(|| {
    //         s1.push('a');
    //     });
    //     s.spawn(|| {
    //         s1.push('a');
    //     });
    // });

    //
    //
    //
    //
    //

    let rc = Arc::new(s1);

    let rc1 = Arc::clone(&rc);
    let rc2 = Arc::clone(&rc);

    let h1 = thread::spawn(move || loop {
        Arc::clone(&rc1);
        dbg!(&rc1);
    });
    let h2 = thread::spawn(move || loop {
        Arc::clone(&rc2);
        dbg!(&rc2);
    });

    h1.join();
    h2.join();

    // // Scoped threads.
    // let s = String::from("hello world");
    // thread::scope(|scope| {
    //     scope.spawn(|| loop {
    //         dbg!(&s);
    //     });
    //     scope.spawn(|| loop {
    //         dbg!(&s);
    //     });
    // });

    // // single-threaded:
    // // Rc<RefCell<T>>

    // // multi-threaded:
    // // Arc<Mutex<T>>

    let s = String::from("hello world");
    let rc = Arc::new(Mutex::new(s));

    let rc1 = Arc::clone(&rc);
    let rc2 = Arc::clone(&rc);

    let h1 = thread::spawn(move || loop {
        Arc::clone(&rc1);
        dbg!(&rc1);
        let mut s_mut_ref = rc1.lock().unwrap();
        String::push(&mut s_mut_ref, '1');
    });
    let h2 = thread::spawn(move || loop {
        Arc::clone(&rc2);
        dbg!(&rc2);
        let mut s_mut_ref = rc2.lock().unwrap();
        String::push(&mut s_mut_ref, '1');
    });

    // thread::scope(|s| {
    //     let s1 = String::from("hello world");
    //     let m = Mutex::new(s1);

    //     let m_ref1 = &m;
    //     let m_ref2 = &m;
    //     let h1 = s.spawn(move || loop {
    //         let mut s_mut_ref = m_ref1.lock().unwrap();
    //         String::push(&mut s_mut_ref, '1');
    //     });
    //     let h2 = s.spawn(move || loop {
    //         let mut s_mut_ref = m_ref2.lock().unwrap();
    //         String::push(&mut s_mut_ref, '1');
    //     });
    //     h1.join();
    //     h2.join();
    //     drop(m);
    // });

    // Communicate through channels
    let (tx, rx) = channel();
    let tx2 = Sender::clone(&tx);
    thread::spawn(move || loop {
        tx.send(10).unwrap();
    });
    thread::spawn(move || loop {
        tx2.send(10).unwrap();
    });
    // loop {
    //     dbg!(rx.recv().unwrap());
    // }

    // let s = String::from("test");
    // let f = || {
    //     drop(s);
    // };
    // f();
    // // can't be called a second time.
    // // f();
    // // dbg!(s);

    // let s = String::from("test");
    // // the move keyword forces f to take ownership of s at the moment the closure is created -> explains why s can't be used afterwards
    // let f = move || {
    //     println!("{}", s);
    // };
    // f();
    // // can be called a second time: why? -> the move happened at closure creation, and then the moved values are stored as fields in the closure (closure owns the values) -> the way we use the fields inside the closure body determines which function trait they implement
    // f();
    // // dbg!(s);

    // // TODO: if we want the result of each thread, how can we do that cleanly with iterators?
}
