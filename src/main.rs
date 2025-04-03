use dotenvy::dotenv;
use std::process::exit;

fn main() {
    dotenv().ok();
    let file_path = match irc::parse_env_args(Vec::new()) {
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
