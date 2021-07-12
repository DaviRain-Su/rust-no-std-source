use example_no_std;
use features_no_std;

fn main() {

    let hello = example_no_std::hello();
    println!("example no std: {}", hello);

    let hello1 = features_no_std::f1();
    println!("features no std: {}", hello1);
}
