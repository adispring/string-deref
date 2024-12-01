struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

fn print_string_ref(s: &String) {
    println!("print_string_ref: {}", s);
}

fn print_str(s: &str) {
    println!("print_str: {}", s);
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
    let my_string = String::from("Hello");
    print_string_ref(&my_string);
    print_str(&my_string);
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}
