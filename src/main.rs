mod context;
mod interp;
mod value;

use std::collections::HashMap;

fn main() {
    println!("\x1b[1m\x1b[34mForth interpreter\x1b[m");

    let mut context = context::Context {
        dict: &mut HashMap::new(),
        stack: &mut Vec::new(),
    };

    context
        .dict
        .insert(String::from("mul2"), String::from("dup + "));

    loop {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).ok();

        let words: Vec<String> = s
            .trim()
            .split_whitespace()
            .map(|e| e.parse().ok().unwrap())
            .collect();

        for word in words {
            interp::interpret(&mut context, word)
        }

        println!("  \x1b[1m\x1b[32mok\x1b[m")
    }
}
