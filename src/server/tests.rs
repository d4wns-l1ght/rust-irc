use crate::server::*;

#[test]
fn parse_prefix() {
    let mut line = ":ecs.vuw.ac.nz JOIN #foo,#bar fubar,foobar".to_owned();
    assert_eq!(
        try_parse_from_line(&mut line).unwrap(),
        Command {
            prefix: Some("ecs.vuw.ac.nz".to_owned()),
            kind: CommandKind::Join {
                channels: vec!["#foo".to_string(), "#bar".to_string()],
                keys: Some(vec!["fubar".to_string(), "foobar".to_string()]),
            }
        }
    );

    let mut line = ":nvx-23!nvx@ecs.vuw.ac.nz JOIN #foo,#bar fubar,foobar".to_owned();
    assert_eq!(
        try_parse_from_line(&mut line).unwrap(),
        Command {
            prefix: Some("nvx-23!nvx@ecs.vuw.ac.nz".to_owned()),
            kind: CommandKind::Join {
                channels: vec!["#foo".to_string(), "#bar".to_string()],
                keys: Some(vec!["fubar".to_string(), "foobar".to_string()]),
            }
        }
    );

    let mut line = ":[{|21lu}]!wilkesluna@192.523.3.21 JOIN #foo,#bar fubar,foobar".to_owned();
    assert_eq!(
        try_parse_from_line(&mut line).unwrap(),
        Command {
            prefix: Some("[{|21lu}]!wilkesluna@192.523.3.21".to_owned()),
            kind: CommandKind::Join {
                channels: vec!["#foo".to_string(), "#bar".to_string()],
                keys: Some(vec!["fubar".to_string(), "foobar".to_string()]),
            }
        }
    );
}

#[test]
#[should_panic]
fn parse_bad_prefix() {
    // too long
    let mut line = ":lunaamethystwilkes JOIN #foo,#bar fubar,foobar".to_owned();
    try_parse_from_line(&mut line).unwrap();

    // digit in first location
    let mut line = ":03luna JOIN #foo,#bar fubar,foobar".to_owned();
    try_parse_from_line(&mut line).unwrap();

    // user without host
    let mut line = ":wilkesluna!abc JOIN #foo,#bar fubar,foobar".to_owned();
    try_parse_from_line(&mut line).unwrap();

    // invalid ip4addr
    let mut line = ":wilkesluna@1.1.1.1.1 JOIN #foo,#bar fubar,foobar".to_owned();
    try_parse_from_line(&mut line).unwrap();

    // invalid ip6addr
    let mut line = ":wilkesluna@019X JOIN #foo,#bar fubar,foobar".to_owned();
    try_parse_from_line(&mut line).unwrap();
}

#[test]
fn parse_join() {
    let mut line = "JOIN #foo,#bar fubar,foobar".to_owned();
    assert_eq!(
        try_parse_from_line(&mut line).unwrap(),
        Command {
            prefix: None,
            kind: CommandKind::Join {
                channels: vec!["#foo".to_string(), "#bar".to_string()],
                keys: Some(vec!["fubar".to_string(), "foobar".to_string()]),
            }
        }
    );

    let mut line = "JOIN 0".to_owned();
    assert_eq!(
        try_parse_from_line(&mut line).unwrap(),
        Command {
            prefix: None,
            kind: CommandKind::Join {
                channels: vec!["0".to_string()],
                keys: None,
            }
        }
    );
}

#[test]
#[should_panic]
fn parse_join_no_params() {
    let mut line = "JOIN".to_owned();
    try_parse_from_line(&mut line).unwrap();
}

#[test]
#[should_panic]
fn parse_join_too_many_params() {
    let mut line = "JOIN #foo,#bar fubar,foobar foooobar".to_owned();
    try_parse_from_line(&mut line).unwrap();
}

#[test]
#[should_panic]
fn parse_invalid_channel() {
    let mut line = "JOIN foo,#bar".to_owned();
    try_parse_from_line(&mut line).unwrap();
}
