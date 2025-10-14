// todo:
// impl Trait for Type {}
// default implementations
// impl Trait syntax (downside: multiple occurences don't have to be same type, doesn't work in type definitions)
// fn<T: Trait>(t: T) or fn<T>(t: T) where T: Trait (bound)
// trait Trait: A + B (any type implementig Trait must also implement A and B)
// "polymorphism": poly=many, morph=form
// polymorphism in rust: generics, enums, dynamic dispatch
// monomorphization (static dispatch)
// associated vs generic type parameter (show Add example)
// trait objects
// object safety

// TODO:
// v.iter() == (&v).into_iter()
// v.iter_mut() == (&mut v).into_iter()
// v.into_iter()

// TODO: error handling
// use anyhow::Result;
fn bar() -> Result<u32, Box<dyn Error>> {
    // fn bar() -> Result<u32, String> {
    let a = "120".parse()?;
    Ok(a)
}

use std::{error::Error, iter, ops::Add};

#[derive(Debug, PartialEq, Default, Copy, Clone)]
struct Int<T: std::fmt::Debug> {
    inner: T,
}

struct Int2 {
    inner: u32,
}

// generic type parameter
impl Add<Int2> for Int<f32> {
    // associated type
    type Output = Int<f32>;

    fn add(self, rhs: Int2) -> Self::Output {
        todo!()
    }
}

// impl Add<u32> for Int {
//     fn add(self, rhs: u32) -> Self::Output {
//         todo!()
//     }
// }

// impl Int {
//     // implement member functions.
// }

// impl Trait for Int {
//     // implement specific member functions == functions of the trait.
// }

// impl Ord for Int {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         todo!()
//     }
// }

// impl PartialOrd for Int {

// }

// trait Add<Lhs, Rhs, Output> {
//     // type Output;

//     fn add(self, rhs: Rhs);
// }

fn foo(iterable: impl IntoIterator<Item = u32>, iterable2: impl IntoIterator<Item = u32>) {
    for i in iterable {
        dbg!(i);
    }
}

// fn iterable(v: Vec<u32>) -> Box<impl Iterator<Item = u32>> {
//     Box::new(v.into_iter())
// }

// fn iterable2(v: Vec<u32>) -> Box<dyn Iterator<Item = u32>> {
//     Box::new(v.into_iter())
// }

// fn iterable3<T>(v: Vec<T>) -> Box<impl Iterator<Item = T>> {
//     Box::new(v.into_iter())
// }

// fn iterable4<T>(v: Vec<T>) -> Box<dyn Iterator<Item = T>> {
//     Box::new(v.into_iter())
// }

fn iterable10<'a>(v1: Vec<impl Default>, v2: Vec<impl Default>) -> impl Default {
    todo!()
}

fn iterable12<'a, A: Default, B: Default, C: Default>(v1: Vec<A>, v2: Vec<B>) -> C {
    todo!()
}

fn iterable11<'a, T: Default>(v1: Vec<T>, v2: Vec<T>) -> T {
    todo!()
}

fn iterable13<'a, T: std::fmt::Debug>(v1: Vec<T>, v2: Vec<T>) -> T
where
    T: Default,
{
    todo!()
}

fn iterable14<'a, T: std::fmt::Debug + Default>(v1: Vec<T>, v2: Vec<T>) -> T {
    todo!()
}

fn iterable15<'a, T>(v1: Vec<T>, v2: Vec<T>) -> T
where
    T: Default + std::fmt::Debug,
{
    todo!()
}

fn iterable5<'a, T: Default>(v1: Vec<T>, v2: Vec<T>) -> impl Iterator<Item = T> {
    // same types (IntoIter)
    if true {
        v1.into_iter()
    } else {
        v2.into_iter()
    }
}

// fn iterable6<'a, T: Default>(v1: Vec<T>, v2: Vec<T>) -> impl Iterator<Item = T> {
//     // different types
//     if true {
//         iter::once(T::default())
//     } else {
//         v1.into_iter()
//     }
// }

// fn iterable7<'a, T: Default>(v1: Vec<T>, v2: Vec<T>) -> Box<impl Iterator<Item = T>> {
//     // different types
//     if true {
//         Box::new(iter::once(T::default()))
//     } else {
//         Box::new(v1.into_iter())
//     }
// }

fn iterable8<'a, T: Default + Clone + 'a>(v: &'a Vec<T>) -> Box<dyn Iterator<Item = T> + 'a> {
    // different types
    if true {
        let a = iter::once(T::default());
        Box::new(a)
    } else {
        let a = v.iter().cloned();
        Box::new(a)
    }
}

// apparently traits have lifetimes
fn iterable9<'a, T: Default + 'a>(v: Vec<T>) -> Box<dyn Iterator<Item = T> + 'a> {
    // different types
    if true {
        Box::new(iter::once(T::default()))
    } else {
        Box::new(v.into_iter())
    }
}

fn foo2<T: IntoIterator<Item = u32>, T2: IntoIterator<Item = u32>>(iterable: T, iterable2: T2) {
    for i in iterable {
        dbg!(i);
    }
}

// fn addable<T: Add<Output = T>>(a: T, b: T) -> T {
//     a + b
// }

fn foo3<T>(iterable: T)
where
    T: IntoIterator<Item = u32> + std::fmt::Debug,
{
    for i in iterable {
        dbg!(i);
    }
}

trait Animal: std::fmt::Debug {
    fn speak(&self);
}

#[derive(Debug)]
struct Cat;
#[derive(Debug)]
struct Dog;

impl Animal for Cat {
    fn speak(&self) {
        println!("i'm a cat");
    }
}

impl Animal for Dog {
    fn speak(&self) {
        println!("i'm a dog");
    }
}

fn animals_speak<T: Animal>(animals: Vec<T>) {
    for animal in animals {
        animal.speak();
    }
}

// advantages: can live on stack, forces handling of all variants, static dispatch enables compiler optimizations
#[derive(Debug)]
enum AnimalEnum {
    Dog(Dog),
    Cat(Cat),
}

impl Animal for AnimalEnum {
    fn speak(&self) {
        match self {
            AnimalEnum::Dog(dog) => dog.speak(),
            AnimalEnum::Cat(cat) => cat.speak(),
        }
    }
}

fn animals_enum_speak(animals: Vec<AnimalEnum>) {
    for animal in animals {
        animal.speak();
    }
}

trait SuperTrait: Animal + std::fmt::Display {}
impl<T: Animal + std::fmt::Display> SuperTrait for T {}

fn main() {
    // OOP
    // animals_speak(vec![Cat {}, Cat {}]);
    // let v: Vec<Cat> = vec![];
    // animals_speak(v);
    // animals_speak(vec![Cat {}, Dog {}]);

    // animals_enum_speak(vec![AnimalEnum::Dog(Dog {}), AnimalEnum::Cat(Cat {})]);

    // let v: Vec<Box<(dyn SuperTrait)>> = vec![Box::new(Cat {}), Box::new(Dog {})];
    let mut cat = Cat {};
    let mut dog = Dog {};
    let v: Vec<&mut dyn Animal> = vec![&mut cat, &mut dog];
    for i in v {
        i.speak();
    }
}
