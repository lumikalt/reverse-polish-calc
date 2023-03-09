#![feature(array_try_map)]

use parser::Env;
use rustyline::{error::ReadlineError, DefaultEditor};

pub mod builtins;
pub mod error;
pub mod parser;

pub fn repl() -> rustyline::Result<()> {
    println!("Reverse Polish Notation Calculator - 1.0");

    let mut rl = DefaultEditor::new()?;
    if rl.load_history("history.txt").is_err() {
        println!("No previous history");
    }

    loop {
        let input = rl.readline("> ");

        match input {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;
                rl.save_history("history.txt")?;

                let result = Env::new(line).interpret();
                println!("{}", result);
            }
            Err(ReadlineError::Interrupted) => {
                eprintln!("C-c");
                break;
            }
            Err(ReadlineError::Eof) => {
                eprintln!("C-d");
                break;
            }
            Err(err) => {
                eprintln!("Error: {}", err);
                break;
            }
        }
    }

    Ok(())
}

// lazy_static! {
//     static ref NUMBER: Regex = Regex::new(r"[-+]?([0-9]*[.])?[0-9]+([eE][-+]?\d+)?").unwrap();
// }
