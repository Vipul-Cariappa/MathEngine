mod lang;
use lang::lexer::Lexer;
use lang::parser::Parser;
use std::{io::Write, process::ExitCode};

fn main() -> ExitCode {
    let prompt = "MathEngine >>> ";
    let prompt_length = prompt.len();

    let mut line = String::new();
    print!("{}", prompt);
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut line).unwrap();

    for i in Lexer::new(line.clone()) {
        match i {
            Ok(x) => print!("{:?}, ", x),
            Err(e) => {
                println!(
                    "\n{}{}\n{}^\nError: {}",
                    prompt,
                    line,
                    " ".repeat(prompt_length + e.position),
                    e.message
                );

                return ExitCode::FAILURE;
            }
        };
    }

    let p = Parser::new(line).parse();
    println!("\n Parser: {}", p.unwrap());

    println!("");
    return ExitCode::SUCCESS;
}
