extern crate rustyline;

use rustyline::error::ReadlineError;
use rustyline::Editor;

fn read(input: String) -> String {
    input
}

fn eval(input: String) -> String {
    input
}

fn print(input: String) -> String {
    input
}

fn rep(input: String) -> String {
    print(eval(read(input)))
}

fn main() {
    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new();
    if rl.load_history(".mal-history").is_err() {
        eprintln!("No previous history.");
    }

    loop {
        let readline = rl.readline("user> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(&line);
                rl.save_history(".mal-history").unwrap();
                if line.len() > 0 {
                    println!("{}", rep(line));
                }
            }
            Err(ReadlineError::Interrupted) => continue,
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}
