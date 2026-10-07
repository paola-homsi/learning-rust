use crate::models::{Action, CmdRequest};

/// Parse command-line arguments (without the program name).
pub fn parse<I, S>(args: I) -> Result<CmdRequest, String>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let mut req = CmdRequest::default();
    let mut args = args.into_iter().map(Into::into);

    while let Some(arg) = args.next() {
        match arg.as_str() {
            // Consume the value here so it is not parsed again as a flag.
            "-n" | "--name" => match args.next() {
                Some(value) => req.file = Some(value),
                None => return Err(format!("{arg} needs a file name")),
            },
            "-s" | "--size" => req.actions.push(Action::Size),
            "-l" | "--lines" => req.actions.push(Action::Lines),
            "-h" | "--help" => req.actions.push(Action::Help),
            other => return Err(format!("unknown argument: {other}")),
        }
    }
    Ok(req)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_name_is_not_treated_as_a_flag() {
        let req = parse(["-n", "notes.txt", "-s", "-l"]).unwrap();
        assert_eq!(req.file.as_deref(), Some("notes.txt"));
        assert_eq!(req.actions, vec![Action::Size, Action::Lines]);
    }

    #[test]
    fn long_flags_work() {
        let req = parse(["--lines", "--name", "a.txt"]).unwrap();
        assert_eq!(req.file.as_deref(), Some("a.txt"));
        assert_eq!(req.actions, vec![Action::Lines]);
    }

    #[test]
    fn missing_file_name_is_an_error_not_a_panic() {
        assert!(parse(["-s", "-n"]).is_err());
    }

    #[test]
    fn unknown_flag_is_an_error() {
        assert_eq!(parse(["-x"]).unwrap_err(), "unknown argument: -x");
    }
}
