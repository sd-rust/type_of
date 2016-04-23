#![feature(core_intrinsics)]

use std::clone::Clone;

macro_rules! type_of {
    ($exp: expr) => (
        format!("{}: {}", stringify!($exp), type_of(&($exp)));
    )
}

macro_rules! print_type_of {
    ($exp: expr) => (
        println!("{}", type_of!($exp));
    )
}

fn type_of<T>(_: &T) -> String {
    unsafe {
        std::intrinsics::type_name::<T>()
    }.to_owned()
}

pub fn reversed3<T>(iter: &mut DoubleEndedIterator<Item = T>) -> Vec<T>
    where T: Clone
{
    println!("[Iter param     ] {}", type_of!(iter));

    let mut v = vec![];

    loop {
        match iter.next_back() {
            Some(val) => v.push(val.clone()),
            None => break,
        }
    }

    v
}

fn main() {
    print_type_of!(32.90);
    print_type_of!(vec!(1, 2, 4)); // prints "collections::vec::Vec<i32>"

    let v0 = (0..10).collect::<Vec<u32>>();

    let mut v0r = v0.clone();
    v0r.reverse();

    let iter = &mut v0.iter();

    println!("[Input iterator ] {}", type_of!(iter));

    let r = reversed3(iter);

    println!("[Output iterator] {}", type_of!(r));

    println!("{:?}", r);
}