#![no_std]

extern crate alloc;
use alloc::string::String;
use alloc::format;

pub fn hello () -> String {
    format!("hello, world")
    // println!("hello world");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
