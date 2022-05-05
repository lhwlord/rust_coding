macro_rules! foo {
    ($arg:expr) => {
        print!("{}", $arg);
    };
}
fn main() {
    println!("Hello, world!");

    foo!("abc");
}
