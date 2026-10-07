use std::fs;

use crate::models::{Action, CmdRequest};

pub const HELP: &str = "usage: ctool -n <file> [-s|--size] [-l|--lines]\n\
  -n, --name   file to inspect\n\
  -s, --size   print the file size in bytes\n\
  -l, --lines  print the number of lines\n\
  -h, --help   show this help";

/// Run the requested actions and return the lines to print.
pub fn execute(request: &CmdRequest) -> Result<Vec<String>, String> {
    if request.actions.is_empty() || request.actions.contains(&Action::Help) {
        return Ok(vec![HELP.to_string()]);
    }
    let path = request
        .file
        .as_deref()
        .ok_or("no file given; use -n <file>")?;
    // Propagate the real error instead of encoding it in the content string.
    let content = fs::read_to_string(path).map_err(|e| format!("cannot read {path}: {e}"))?;

    Ok(request
        .actions
        .iter()
        .map(|action| match action {
            Action::Size => format!("size: {} bytes", content.len()),
            Action::Lines => format!("lines: {}", content.lines().count()),
            Action::Help => unreachable!(),
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn temp_file(content: &str) -> String {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "ctool-test-{}-{}.txt",
            std::process::id(),
            content.len()
        ));
        fs::File::create(&path)
            .unwrap()
            .write_all(content.as_bytes())
            .unwrap();
        path.to_string_lossy().into_owned()
    }

    #[test]
    fn counts_lines_without_off_by_one() {
        let file = temp_file("a\nb\n");
        let req = CmdRequest {
            file: Some(file),
            actions: vec![Action::Lines, Action::Size],
        };
        assert_eq!(execute(&req).unwrap(), vec!["lines: 2", "size: 4 bytes"]);
    }

    #[test]
    fn file_starting_with_error_is_still_read() {
        let file = temp_file("error: this is fine\n");
        let req = CmdRequest {
            file: Some(file),
            actions: vec![Action::Lines],
        };
        assert_eq!(execute(&req).unwrap(), vec!["lines: 1"]);
    }

    #[test]
    fn missing_file_is_reported() {
        let req = CmdRequest {
            file: Some("does/not/exist".into()),
            actions: vec![Action::Size],
        };
        assert!(execute(&req).unwrap_err().starts_with("cannot read"));
    }
}
