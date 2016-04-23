#[macro_use] 
extern crate type_of;

use type_of::*;

fn main() {

    assert_eq!("f64", type_of!(32.90));
    assert_eq!("[i32; 3]", type_of!([1, 2, 4]));
    assert_eq!("std::vec::Vec<i32>", type_of!(vec![1, 2, 4]));
    assert_eq!("&'static str", type_of!("123"));

    assert_eq!("32.90: f64", format_type_of!(32.90));
    assert_eq!("[1, 2, 4]: [i32; 3]", format_type_of!([1, 2, 4]));
    assert_eq!("vec!(1 , 2 , 4): std::vec::Vec<i32>", format_type_of!(vec![1, 2, 4]));
    assert_eq!("\"123\": &'static str", format_type_of!("123"));

    print_type_of!(32.90);
    print_type_of!([1, 2, 4]);
    print_type_of!(vec![1, 2, 4]);
    print_type_of!("123");
}