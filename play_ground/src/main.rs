/* Rust PlayGround */

mod question;
mod hello;

fn main() {
    // 他ファイルの関数実行
    hello::print_hello();
    question::hello_question();
    question::q1::q1_hello();
    // 他ファイルのマクロ実行
    say_hello!();
    println!("Hello, world!");
}
