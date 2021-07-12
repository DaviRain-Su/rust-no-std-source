#![no_std]


pub fn sum(a: i32, b: i32) -> i32 {
    a + b
}


pub fn get_hello_string() -> String {
    format!("hello")
}



#[cfg(test)]
mod tests {
    use  super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_sum() {
        assert_eq!(sum(2, 3), 5);
    }
}
