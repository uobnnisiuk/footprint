use std::env;
use std::io::{self, Read};
use std::process;

fn main() {
    let mut stdin_json = String::new();
    if let Err(e) = io::stdin().read_to_string(&mut stdin_json) {
        let err = footprint_core::CoreError {
            code: "io_error",
            message: format!("failed to read stdin: {e}"),
        };
        println!("{}", footprint_core::render_error_json(&err));
        process::exit(1);
    }

    match footprint_core::run(env::args_os(), &stdin_json) {
        Ok(output) => {
            println!("{}", footprint_core::render_success_json(&output));
            process::exit(0);
        }
        Err(err) => {
            println!("{}", footprint_core::render_error_json(&err));
            process::exit(1);
        }
    }
}
