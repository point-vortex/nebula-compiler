use std::env;

mod tokenizer;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
