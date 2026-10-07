mod executor;
mod models;
mod parser;

use std::process::ExitCode;

fn main() -> ExitCode {
    let result = parser::parse(std::env::args().skip(1)).and_then(|req| executor::execute(&req));
    match result {
        Ok(lines) => {
            for line in lines {
                println!("{line}");
            }
            ExitCode::SUCCESS
        }
        Err(message) => {
            eprintln!("error: {message}\n\n{}", executor::HELP);
            ExitCode::from(2)
        }
    }
}
