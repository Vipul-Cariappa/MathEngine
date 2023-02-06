use math_engine;
use std::process::ExitCode;

fn main() -> ExitCode {
    println!("{}", math_engine::get_version());
    ExitCode::SUCCESS
}
