// Ideas on what to cover: I assume most people coming to the lecture are not
// super familiar with Rust, but curious and want to learn. I assume they know
// about other programming languages and its some of its strengths and
// weaknesses. My goal for this lecture is to showcase concrete examples where
// Rust makes programming nicer, mostly related to safety or speed. Basically, I
// don't want to go through Rust feature by feature, but example by example,
// which should get you interested in how things work the way they do, which you
// can then do more research on in your own time.

// Main takeaways i want ppl to go away with:
// - rust improves local reasoning (you can understand sections of code viewed
//   in isolation without knowing where they are called) -> great for
//   abstraction, safety, correct code
// - rust enums are a great way to model data
// - rust's perceived "complexity" is purely compile-time, e.g. ownership,
//   lifetimes, mutable borrowing, generics
// - rust's speed mostly comes from zero cost abstractions (e.g. iterators,
//   generics (monomorphization))

// Examples/use-cases/case studies:

// ownership: simulate immutable and mutable referencing without &/&mut at
// first, then introduce them alongside the rules: ownership rules:
// - Each value in Rust has **one** owner.
// - There can only be one owner at a time.
// - When the owner goes out of scope, the value will be dropped. How is memory
// safety achieved without a GC, not reference counting everything and with no
// explicit malloc/free?! (mention Arc for multi-threaded reference counting)

// copy/clone traits for shallow (bitwise) vs deep copy

// errors as values (no exceptions): Option<T>, Result<T, E>
// go from:
// fn vec_get(v: Vec<u32>, idx: usize, ret_val: &&u32) -> i32
// to:
// fn vec_get(v: Vec<u32>, idx: usize) -> Option<&T>
// we'll define Option<T> ourselves

// lead into enums and pattern matching (rust forces you to handle all cases)
// compare enum dispatch to dynamic dispatch (Box<dyn Trait>)

// if, match are expressions:
// - benefit: can't forget to initialise in one branch (compiler will usually
//   warn you though)
// - improves local reasoning ability

// error propagation with ? operator

// demo iterator invalidation (mutating a collection we're iterating over)

// entry api: only safe to provide this abstraction to users because &mut
// guarantees no mutation possible that could invalidate slot identified by hash

// fn traits for defining functions on our custom option type

// show cool iterator stuff (python default dict in rust): show what it would
// boil down to (e.g. in C)

// demo thread safety (probably out of scope): summary: rust makes
// multithreading safe by, for each closure, keeping track of references to
// captured objects (imm and mut), and then being able to mark the closures
// themselves as having an imm/mut reference to objects; the thread spawn then
// makes that lifetime until the thread ends (maybe never), and that prevents
// multiple threads with concurrent mutable access: captured types must impl
// Send/Sync traits, &mut T doesn't impl those

// things we'll probably not have the time to cover: traits, closures, unsafe

// Useful tips:
// - all build files (many GBs worth!) stored in one dir (easy to delete):
//   `export CARGO_TARGET_DIR="$HOME/cargo-targets"`

fn main() {
    println!("Hello, world!");
    let x = [1, 2, 3]
        .into_iter()
        .map(|x| 5 * x)
        .filter(|x| x % 2 == 0)
        .collect::<Vec<u32>>();
}
