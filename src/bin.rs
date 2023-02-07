use math_engine;
use std::process::ExitCode;

fn main() -> ExitCode {
    println!("MathEngine::Version {}", math_engine::get_version());

    math_engine::do_something();

    ExitCode::SUCCESS
}
