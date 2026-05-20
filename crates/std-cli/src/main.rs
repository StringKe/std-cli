fn main() {
    match std_cli::run_cli(std::env::args_os()) {
        Ok(output) => {
            if !output.is_empty() {
                println!("{output}");
            }
        }
        Err(err) => {
            eprintln!("FAIL {err}");
            std::process::exit(1);
        }
    }
}
