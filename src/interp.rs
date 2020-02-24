use crate::context::Context;
use crate::value::Value;

pub fn interpret(ctx: &mut Context, word: String) {
    let word = word.to_lowercase();

    if let Ok(i) = word.parse::<i32>() {
        ctx.stack.push(Value::Int(i));
        return;
    }

    if let Ok(f) = word.parse::<f64>() {
        ctx.stack.push(Value::Float(f));
        return;
    }

    if word == "." {
        let elem = ctx.stack.pop().expect("Stack underflow (.)");
        print!("{}", elem.to_string());
        return;
    }

    if word == ".s" {
        println!(
            "<{}>{}",
            ctx.stack.len(),
            ctx.stack
                .into_iter()
                .fold(String::new(), |s, e| s + " " + &e.to_string())
        );
        return;
    }

    if word == "+" {
        let rhs = ctx.stack.pop().expect("Stack underflow (+)");
        let rhs = match rhs {
            Value::Int(i) => i,
            Value::Float(_) => panic!("Type mismatch (+)"),
        };

        let lhs = ctx.stack.pop().expect("Stack underflow (+)");
        let lhs = match lhs {
            Value::Int(i) => i,
            Value::Float(_) => panic!("Type mismatch (+)"),
        };

        ctx.stack.push(Value::Int(lhs + rhs));
        return;
    }

    if word == "dup" {
        let elem = ctx.stack.last().expect("Stack underflow (dup)").clone();
        ctx.stack.push(elem);
        return;
    }

    if word == "drop" {
        ctx.stack.pop().expect("Stack underflow (drop)");
        return;
    }

    if word == "bye" {
        std::process::exit(0)
    }

    if let Some(word_def) = ctx.dict.get(&word) {
        let words: Vec<String> = word_def
            .trim()
            .split_whitespace()
            .map(|e| e.parse().ok().unwrap())
            .collect();

        for w in words {
            interpret(ctx, w)
        }

        return;
    }

    panic!("Unknown word")
}
