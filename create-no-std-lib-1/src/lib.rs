#![no_std]
extern crate alloc;
use alloc::string::String;
use alloc::format;

pub fn sum(a: i32, b: i32) -> i32 {
    a + b
}


pub fn get_hello_string(value: &str) -> String {
    String::from(value)
}



#[cfg(test)]
mod tests {
    use  super::*;
    use alloc::string::String;
    use alloc::string::ToString;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_sum() {
        assert_eq!(sum(2, 3), 5);
    }

    #[test]
    fn test_get_hello_string() {
        assert_eq!("hello".to_string(), get_hello_string("hello"));
    }
}
