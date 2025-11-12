// TODO:
// unsafe is superset of safe: adds on unsafe superpowers
// sound programs: no UB, is correct
// safe programs: subset of all sound programs, since compiler might reject some sound programs (compiler guarantees soundness though)
// unsafe programs: can be sound or unsound, depends on programmer instead of compiler; keep potentially unsound code in marked regions
// safe abstractions over unsafe implementations
// unsafe functions vs unsafe blocks: unfortunate naming, different meaning: blocks: "let me call unsafe functions in here", function: "mark this function as unsafe"
// 5 unsafe superpowers:
// 1. dereference raw pointers
// 2. call unsafe functions
// 3. implement unsafe traits
// 4. read/write a mutable global variable
// 5. reading from a field of a union (untagged enum: enum that doesn't know which value it is) is unsafe (surprisingly, writing is safe)
// make unsafe blocks as small as possible

/// Safety: The caller must ensure that ptr is valid and points to a valid i32.
unsafe fn increment_value(ptr: *mut i32) {
    unsafe {
        *ptr += 1;
    }
}

// TODO: don't do this!!! if we know we're mutating, might as well use *mut so type system can help with some stuff
/// Safety: The caller must ensure that ptr is valid and points to a valid i32.
unsafe fn increment_value2(ptr: *const i32) {
    // cast
    let ptr: *mut i32 = ptr as *mut i32;
    *ptr += 1;
}

// TODO: don't do this!!! user will assume it's safe to call because it's marked as safe.
/// Safety: The caller must ensure that ptr is valid and points to a valid i32.
unsafe fn increment_value3(ptr: *mut i32) {
    unsafe {
        *ptr += 1;
    }
}

// unsound!
fn safe() {
    let i = 10;
    let ptr: *mut i32 = i as *mut i32;
    // Safety: *mut i is valid and points to valid i32.
    unsafe {
        increment_value(ptr);
    }
    dbg!(i);
}

// sound
fn safe2() {
    let mut i = 10;
    let ptr: *mut i32 = &mut i;
    // Safety: *mut i is valid and points to valid i32.
    unsafe {
        increment_value(ptr);
    }
    dbg!(i);
}

fn main() {
    println!("Hello, world!");
    safe();
    // safe2();

    unsafe {
        let mut s = String::new();
        let s_ptr: *mut String = &mut s;
        // let s_clone = *s_ptr;
        // let s_clone2 = *s_ptr;
        drop(s);
    }
}
