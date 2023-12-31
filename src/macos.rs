use std::{error::Error, process::Command};

pub fn put_path_into_clipboard(path: &str) -> Result<(), Box<dyn Error>> {
    let osascript_source = format!(
        "set the clipboard to \"{}\" as «class furl»",
        prepare_apple_script_string_literal(path)
    );
    Command::new("osascript")
        .args(["-e", osascript_source.as_str()])
        .output()?;
    Ok(())
}

fn prepare_apple_script_string_literal(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_prepare_apple_script_string_literal() {
        let f = super::prepare_apple_script_string_literal;
        assert_eq!("hello", f("hello"));
        assert_eq!("", f(""));
        assert_eq!("\\\"hello\\\"", f("\"hello\""));
    }
}
