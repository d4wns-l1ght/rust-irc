use crate::server::*;

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
