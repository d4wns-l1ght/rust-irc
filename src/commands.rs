#[derive(Debug, PartialEq)]
pub struct Command {
    pub prefix: Option<String>,
    pub kind: CommandKind,
}

#[derive(Debug, PartialEq)]
pub enum CommandKind {
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
        source_server: String,
        target_server: Option<String>,
    },
    PrivMsg {
        message_target: String,
        message_text: String,
    },
    Quit {
        quit_message: Option<String>,
    },
}
