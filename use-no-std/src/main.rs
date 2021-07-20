use create_no_std_lib_1::get_hello_string;
use create_no_std_lib_2::hello;

fn main() {
    // test no-std create no std lib 1
    let temp = get_hello_string("hello");
    println!("temp = {}",temp);

    let hello = hello("hello world");
    println!("hello = {:?}", hello);


}
