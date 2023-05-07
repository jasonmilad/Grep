use std::env;
use std::process;
use grep::Params;




fn main() {

    let args: Vec<String> = env::args().collect();
    let params = Params::new(&args).unwrap_or_else(|err| {
        eprintln!("Error message: {}", err);
        process::exit(1);
    });

    if let Err(e) = grep::run(params) {
        eprintln!("Error message: {e}");
        process::exit(1);
    };

}