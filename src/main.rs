mod intorfloat;
mod lexer;
mod token;

use std::process::ExitCode;
use lexer::Lexer;

use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

fn run(line: &str) -> Result<(), String> {
    let mut lexer = Lexer::new(line);
    let tokens = lexer.lex()?;
    println!("{:?}", tokens);
    Ok(())
}

fn interactive_mode() -> rustyline::Result<()> {
    let mut rl = DefaultEditor::new()?;
    loop {
        let result = rl.readline(">>> ");
        match result {
            Ok(str) => {
                let trimmed_str = str.trim();
                if trimmed_str.len() == 0 {
                    continue;
                }
                let _ = rl.add_history_entry(trimmed_str);
                match run(trimmed_str) {
                    Ok(_) => (),
                    Err(err_str) => {
                        eprintln!("{}", err_str)
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C")
            },
            Err(ReadlineError::Eof) => {
                println!("^D");
                break
            },
            Err(err) => {
                eprintln!("Readline error: {:?}", err);
                return Err(err);
            },
        }
    }
    Ok(())
}

fn main() -> ExitCode {
    match interactive_mode() {
        Ok(_) => ExitCode::SUCCESS,
        Err(_) => ExitCode::FAILURE,
    }
}
