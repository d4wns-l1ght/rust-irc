use thiserror::Error;

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum IrcError {
    #[error("{nickname:?} :No such nick/channel")]
    NoSuchNick { nickname: String },
    #[error("{server:?} :No such server")]
    NoSuchServer { server: String },
    #[error("{channel:?} :No such channel")]
    NoSuchChannel { channel: String },
    #[error("{channel:?} :Cannot send to channel")]
    CannotSendToChan { channel: String },
    #[error("{channel:?} :You have joined too many channels")]
    TooManyChannels { channel: String },
    #[error("{nickname:?} :There was no such nickname")]
    WasNoSuchNick { nickname: String },
    #[error("{target:?} :Duplicate recipients. No message delivered")]
    TooManyTargets { target: String },
    #[error(":No origin specified")]
    NoOrigin,
    #[error(":No recipient given (?:command)")]
    NoRecipient { command: String },
    #[error("No text to send")]
    NoTextToSend,
    #[error("{mask:?} :No toplevel domain specified")]
    NoTopLevel { mask: String },
    #[error("{mask:?} :Wildcard in toplevel domain")]
    WildTopLevel { mask: String },
    #[error("{command:?} :Unknown command")]
    UnknownCommand { command: String },
    #[error(":MOTD File is missing")]
    NoMotd,
    #[error("{server:?} :No administrative info available")]
    NoAdminInfo { server: String },
    #[error(":File error doing (?:file_op) on (?:file)")]
    FileError { file_op: String, file: String },
    #[error(":No nickname given")]
    NoNickNameGiven,
    #[error("{nickname:?} :Erroneus nickname")]
    ErroneusNickname { nickname: String },
    #[error("{nickname:?} :Nickname is already in use")]
    NicknameInUse { nickname: String },
    #[error("{nickname:?} :Nickname collision KILL")]
    NickCollision { nickname: String },
    #[error("{nickname:?} {channel} :They aren't on that channel")]
    UserNotInChannel { nickname: String, channel: String },
    #[error("{channel:?} :You're not on that channel")]
    NotOnChannel { channel: String },
    #[error("{user:?} {channel:?} :is already on channel")]
    UserOnChannel { user: String, channel: String },
    #[error("{user:?} :User not logged in")]
    NoLogin { user: String },
    #[error(":SUMMON has been disabled")]
    SummonDisabled,
    #[error(":USERS has been disabled")]
    UsersDisabled,
    #[error(":You have not registered")]
    NotRegistered,
    #[error("{command:?} :Not enough parameters")]
    NeedMoreParams { command: String },
    #[error(":You may not reregister")]
    AlreadyRegistered,
    #[error(":Your host isn't among the privileged")]
    NoPermForHost,
    #[error(":Password incorrect")]
    PasswdMismatch,
    #[error(":You are banned from this server")]
    YoureBannedCreep,
    #[error("{channel:?} :Channel key already set")]
    KeySet { channel: String },
    #[error("{channel:?} :Channot join channel (+l)")]
    ChannelIsFull { channel: String },
    #[error("{char:?} :is unknown mode char to me")]
    UnknownMode { char: char },
    #[error("{channel:?} :Cannot join channel (+i)")]
    InviteOnlyChan { channel: String },
    #[error("{channel:?} :Cannot join channel (+b)")]
    BannedFromChan { channel: String },
    #[error("{channel:?} :Channot join channel (+k)")]
    BadChannelKey { channel: String },
    #[error(":Permission Denied - You're not an IRC operator")]
    NoPrivileges,
    #[error("{channel:?} :You're not channel operator")]
    ChanOPrivsNeeded { channel: String },
    #[error(":You cant kill a server!")]
    CantKillServer,
    #[error(":No O-lines for your host")]
    NoOperHost,
    #[error(":Unknown MODE flag")]
    UModeUnkownFlag,
    #[error(":Cant change mode for other users")]
    UsersDontMatch,
}

#[allow(dead_code)]
impl IrcError {
    pub fn numeric_code(&self) -> i16 {
        match *self {
            IrcError::NoSuchNick { .. } => 401,
            IrcError::NoSuchServer { .. } => 402,
            IrcError::NoSuchChannel { .. } => 403,
            IrcError::CannotSendToChan { .. } => 404,
            IrcError::TooManyChannels { .. } => 405,
            IrcError::WasNoSuchNick { .. } => 406,
            IrcError::TooManyTargets { .. } => 407,
            IrcError::NoOrigin => 409,
            IrcError::NoRecipient { .. } => 411,
            IrcError::NoTextToSend => 412,
            IrcError::NoTopLevel { .. } => 413,
            IrcError::WildTopLevel { .. } => 414,
            IrcError::UnknownCommand { .. } => 421,
            IrcError::NoMotd => 422,
            IrcError::NoAdminInfo { .. } => 423,
            IrcError::FileError { .. } => 424,
            IrcError::NoNickNameGiven => 431,
            IrcError::ErroneusNickname { .. } => 432,
            IrcError::NicknameInUse { .. } => 433,
            IrcError::NickCollision { .. } => 436,
            IrcError::UserNotInChannel { .. } => 441,
            IrcError::NotOnChannel { .. } => 442,
            IrcError::UserOnChannel { .. } => 443,
            IrcError::NoLogin { .. } => 444,
            IrcError::SummonDisabled => 445,
            IrcError::UsersDisabled => 446,
            IrcError::NotRegistered => 451,
            IrcError::NeedMoreParams { .. } => 461,
            IrcError::AlreadyRegistered => 462,
            IrcError::NoPermForHost => 463,
            IrcError::PasswdMismatch => 464,
            IrcError::YoureBannedCreep => 465,
            IrcError::KeySet { .. } => 467,
            IrcError::ChannelIsFull { .. } => 471,
            IrcError::UnknownMode { .. } => 472,
            IrcError::InviteOnlyChan { .. } => 473,
            IrcError::BannedFromChan { .. } => 474,
            IrcError::BadChannelKey { .. } => 475,
            IrcError::NoPrivileges => 481,
            IrcError::ChanOPrivsNeeded { .. } => 482,
            IrcError::CantKillServer => 483,
            IrcError::NoOperHost => 491,
            IrcError::UModeUnkownFlag => 501,
            IrcError::UsersDontMatch => 502,
        }
    }
}
