use std::process::exit;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_path = match irc::parse_env_args(args) {
        Ok(file_path) => file_path,
        Err(e) => {
            println!("Error: {e}");
            exit(-1);
        }
    };


    if let Err(e) = irc::run(file_path) {
        println!("Error: {e}");
        exit(-1);
    }
}
