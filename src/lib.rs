use std::fs::File;
use std::io::Write;

///**write_file** writes the string contents in data to the file pointed to by the file parameters.
///
/// # A rguments:
/// file: is the path of the file we want to write to
/// data: the string data we want to write into the file, pointed to by file parameter
///
/// # Returns
/// an io::Result
pub fn write_file(file: &str, data: &str) -> std::io::Result<()> {
    let mut file = File::create(file)?;
    file.write_all(data.as_bytes())?;
    file.sync_all()?;
    Ok(())
}

///replace_tokens will find all the keys listed in the json formatted vars
/// parameter and replace those with the values associated in the json vars.
/// The values can be encoded using:
///     txt (no encoding)
///     html (html escaping)
///
/// # Parameters:
/// source: contains the string with the keys (tokens) that will be replaced
/// vars: contains the json string documenting a list in key value pair like:
///
///{
///   "vars": {
///      "%env%": "D",
///      "THIS": "that"
///   }
///}
///
/// encode_as: can contain the string html (for html escaping) or txt (for no esaping/encoding)
///
/// # Returns:
/// A new string with the replaced keys (tokens)
pub fn replace_tokens(source: &str, vars: &str, encode_as: &str) -> String {
    let mut result: String;
    result = source.to_string();

    let json: serde_json::Value = serde_json::from_str(&vars).expect("JSON malformed");

    if let Some(field) = json.get("vars") {
        for (key, _value) in field.as_object().unwrap() {
            match encode_as {
                "html" => {
                    result = result.replace(
                        key,
                        &html_escape::encode_text(json["vars"][key].as_str().unwrap()),
                    )
                }
                _ => result = result.replace(key, json["vars"][key].as_str().unwrap()),
            }
        }
    } else {
        eprintln!("ABORT: Your vars file should contain a field: vars");
        std::process::exit(1);
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_write_file() {
        let result = write_file("test.unittest", "Testing");
        match result {
            Ok(()) => assert_eq!(1, 1),
            _ => assert_eq!(1, 0),
        }

        let contents = std::fs::read_to_string("test.unittest".to_string()).unwrap();
        assert_eq!(contents, "Testing");

        let result = std::fs::remove_file("test.unittest");
        match result {
            Ok(()) => assert_eq!(1, 1),
            _ => assert_eq!(1, 0),
        }
    }

    #[test]
    fn test_replace_tokens() {
        let json: &str = "{ \"vars\": { \"%env%\": \"D\", \"THIS\": \"<this>\"}}";
        let mut template: &str = "Where you see %env% it should say D";

        let result = replace_tokens(&template, &json, &String::from("txt"));
        assert_eq!(result.contains("Where you see D it should say D"), true);

        template = "Where we see THIS it should say this";
        let mut result = replace_tokens(&template, &json, &String::from("txt"));
        assert_eq!(
            result.contains(r#"Where we see <this> it should say this"#),
            true
        );

        template = "THIS";
        result = replace_tokens(&template, &json, &String::from("html"));
        assert_eq!(result.contains(r#"&lt;this&gt;"#), true);
    }
}
