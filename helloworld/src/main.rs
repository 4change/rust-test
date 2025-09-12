use std::env;

fn main() {
        let name = env::args().skip(1).next();
        match name {
            Some(n) => println!("Hello {}", n),
            None => println!("Please00000000000000000000---------------- use ./hellowolrd name.")
        }
    }