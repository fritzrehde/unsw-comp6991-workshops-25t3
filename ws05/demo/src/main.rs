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

use std::ops::Add;

#[derive(Debug, Clone, Copy)]
struct Foo {
    int: i32,
}

// enum Option<T> {
//     Some(T),
//     None,
// }

// trait MyDebug {
//     fn debug_print(&self) -> Result<(), ()>;

//     fn debug_print2(&self) -> Result<(), ()> {
//         dbg!(self);
//         Ok(())
//     }
// }

// impl std::fmt::Debug for Foo {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("Foo").field("int", &self.int).finish()
//     }
// }

// impl MyDebug for Foo {
//     fn debug_print(&self) -> Result<(), ()> {
//         todo!()
//     }

//     fn debug_print2(&self) -> Result<(), ()> {
//         todo!()
//     }
// }

fn dbg_every_elem_in_vec<T>(v: Vec<T>)
where
    T: std::fmt::Debug + std::fmt::Display,
{
    for e in v {
        dbg!(&e);
        println!("{}", e);
    }
}

fn dbg_every_elem_in_vec_box_u32(v: Vec<Box<u32>>) {
    // fn dbg_every_elem_in_vec<T>(v: Vec<T>) {
    for e in v {
        dbg!(&e);
        println!("{}", e);
    }
}

struct MyI32(i32);

// struct Vec(VecDeque);

//  impl Vec {

//  }

impl Add<i32> for MyI32 {
    type Output = MyI32;

    fn add(self, rhs: i32) -> Self::Output {
        MyI32(rhs + self.0)
    }
}

impl Add<u32> for MyI32 {
    type Output = MyI32;

    fn add(self, rhs: u32) -> Self::Output {
        MyI32((rhs as i32) + self.0 as i32)
    }
}

// impl Add<i32> for MyI32 {
//     // trait implementor decides the return type
//     type Output = i32;

//     fn add(self, rhs: i32) -> Self::Output {
//         todo!()
//     }
// }

trait MyAdd<Rhs, Ret> {
    fn my_add(a: Self, b: Rhs) -> Ret;
}

impl MyAdd<i32, i32> for i32 {
    fn my_add(a: Self, b: i32) -> i32 {
        a + b
    }
}

impl MyAdd<i32, u32> for i32 {
    fn my_add(a: Self, b: i32) -> u32 {
        (a + b) as u32
    }
}

// trait Supertrait: Animal + std::fmt::Debug {}

trait Animal {
    fn speak(&self) {
        println!("i am the default implementation");
    }
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn speak(&self) {
        println!("i am a dog");
    }
}
impl Animal for Cat {
    fn speak(&self) {
        println!("i am a cat");
    }
}

fn all_animals_speak<T: Animal>(animals: Vec<T>) {
    for animal in animals {
        animal.speak();
    }
}

fn all_animals_speak_dyn<'a, T>(animals: T)
where
    T: Iterator<Item = &'a dyn Animal>,
{
    for animal in animals {
        animal.speak();
    }
}

fn all_animals_speak_concrete(animals: Vec<ConcreteAnimal>) {
    for animal in animals {
        animal.speak();
    }
}

impl<T: Animal> Animal for Box<T> {
    fn speak(&self) {
        let animal = self.as_ref();
        animal.speak();
    }
}

enum ConcreteAnimal {
    Dog(Dog),
    Cat(Cat),
}

impl Animal for ConcreteAnimal {
    fn speak(&self) {
        match self {
            ConcreteAnimal::Dog(dog) => dog.speak(),
            ConcreteAnimal::Cat(cat) => cat.speak(),
        }
    }
}

fn main() {
    // TODO: OOP with animals
    // let animals = vec![Dog, Cat];
    // all_animals_speak(animals);

    let animals: Vec<Box<dyn Animal>> = vec![Box::new(Dog), Box::new(Cat)];
    // all_animals_speak_dyn(animals.iter().map(|box_| box_.as_ref()));
    // all_animals_speak_dyn(animals.iter().map(|box_| box_.as_ref()));

    let animals = vec![ConcreteAnimal::Dog(Dog), ConcreteAnimal::Cat(Cat)];
    all_animals_speak_concrete(animals);

    let animal: Box<dyn Animal> = Box::new(Dog);
    let animal: &dyn Animal = &Dog;

    // dbg_every_elem_in_vec::<u32>(vec![1, 2, 3]);
    // MyI32(10).add::<i32>(10);
    // let res = MyI32(10).add(10u32);
    // let res = MyI32(10).add(10i32);

    // forced to annotate x's type
    // let x = 10 + 20;

    // let a: i32 = 10;
    // let b: i32 = 10;
    // let c: i32 = MyAdd::my_add(a, b);
}
