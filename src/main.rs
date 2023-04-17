mod lang;
use lang::interpret;
use std::{io::Write, process::ExitCode};

fn main() -> ExitCode {
    let prompt = "MathEngine >>> ";

    loop {
        let mut line = String::new();
        print!("{}", prompt);
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut line).unwrap();

        if line == "exit\n" || line == "quit\n" {
            return ExitCode::SUCCESS;
        }

        match interpret(line) {
            Ok(r) => println!(" |> {}", r),
            Err(e) => println!("{}", e),
        };
    }
}
