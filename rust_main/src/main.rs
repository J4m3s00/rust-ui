use c_interface::hello_world;

fn main() {
    println!("Hello, world! From rust_main");
    let x = hello_world();
    println!("Got {}", x);
}
