#![feature(core_intrinsics)]

#[macro_export]
macro_rules! type_of {
    ($exp: expr) => (
        type_of(&($exp))
    )
}

#[macro_export]
macro_rules! print_type_of {
    ($exp: expr) => (
        println!("{}: {}", stringify!($exp), type_of!($exp));
    )
}

#[macro_export]
macro_rules! format_type_of {
    ($exp: expr) => (
        format!("{}: {}", stringify!($exp), type_of!($exp));
    )
}

pub fn type_of<T>(_: &T) -> String {
    unsafe {
        std::intrinsics::type_name::<T>()
    }.to_owned()
}

#[cfg(test)]
mod test {

    use super::*;
    
    #[test]    
    fn basic_test() {
        print_type_of!(32.90);
        print_type_of!([1, 2, 4]);
        print_type_of!(vec![1, 2, 4]);
        print_type_of!("123");

        assert_eq!("32.90: f64", format_type_of!(32.90));
        assert_eq!("[1, 2, 4]: [i32; 3]", format_type_of!([1, 2, 4]));
        assert_eq!("vec!(1 , 2 , 4): std::vec::Vec<i32>", format_type_of!(vec![1, 2, 4]));
        assert_eq!("\"123\": &'static str", format_type_of!("123"));
    }    
}

