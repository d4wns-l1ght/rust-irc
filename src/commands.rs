use std::net::IpAddr;


pub enum Command {
    Join {
        channels: Vec<String>,
        keys: Option<Vec<String>>,
    },
    Nick {
        nickname: String,
    },
    User {
        username: String,
        full_name: String,
    },
    Ping {
        source_server: IpAddr,
        target_server: Option<IpAddr>,
    },
    PrivMsg {
        message_target: String,
        message_text: String,
    },
    Quit {
        quit_message: Option<String>,
    },
}
