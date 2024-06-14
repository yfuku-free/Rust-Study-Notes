#[macro_export]
macro_rules! say_hello {
    () => {
        println!("Hello, world!");
    };
}

pub fn q1_hello() {
    println!("q1");
    say_hello!();
}

