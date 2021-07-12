#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

// run std env
#[cfg(feature = "std")]
pub fn func() -> String {
    format!("func std : hello world")
}

// run no_std env
#[cfg(not(feature = "std"))]
pub fn func() -> alloc::string::String {
    alloc::format!("func no_std : hello world")
}





#[cfg(test)]
mod tests {
    use super::*;
    use alloc::format;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }


    /// run std success
    // run no_std failed
    #[test]
    fn test_func() {
        assert_eq!(format!("func std : hello world"), func());
    }
}
