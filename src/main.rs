extern crate html_escape;
use std::env;
use std::fs::{self, File};
use std::io::Write;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    check_args(&args);

    let source = open_file(&args[1]);
    let vars = open_file(&args[2]);
    let encode_as = &args[3];
    let replaced = replace_tokens(&source, &vars, &encode_as.to_string());

    println!("{}", replaced);

    if args.len() == 4 {
        return write_file(&args[1], &replaced);
    } else {
        return write_file(&args[4], &replaced);
    }
}

///**write_file** writes the string contents in data to the file pointed to by the file parameters.
///
/// # Parameters:
///     file: is the path of the file we want to write to
///     data: the string data we want to write into the file, pointed to by file parameter
///
/// # Returns
///     an io::Result
fn write_file(file: &str, data: &String) -> std::io::Result<()> {
    let mut file = File::create(file)?;
    file.write_all(data.as_bytes())?;
    file.sync_all()?;
    Ok(())
}

/// **open_file** will check if the file pointed to by the file parameters exists, if so it will read the contents of that file
/// and return it to the caller. This is mainly used for text files!
/// When the file can't be found, then we exit! As there's no further use in this application to continue.
/// Usually you would let the main application make this decision, but for sake of code reduction, the exit is done by this function
///
/// # Parameters:
///     file: is the path of the file we want to write to
///
/// # Returns:
///     the contants from the file if succeeded.
fn open_file(file: &str) -> String {
    let result: String;

    if std::path::Path::new(&file).exists() {
        result = fs::read_to_string(file.to_string()).expect("Could not open file");
        return result;
    }

    eprintln!("File not found: {}", file);
    std::process::exit(1);
}

///replace_tokens will find all the keys listed in the json formatted vars
/// parameter and replace those with the values associated in the json vars.
/// The values can be encoded using:
///     txt (no encoding)
///     html (html escaping)
///
/// # Parameters:
///     source: contains the string with the keys (tokens) that will be replaced
///     vars: contains the json string documenting a list in key value pair like:
/// ```json
///{
///   "vars": {
///      "%env%": "D",
///      "THIS": "<this>"
///   }
///}
/// ```
/// encode_as: can contain the string html (for html escaping) or txt (for no esaping/encoding)
///
/// # Returns:
/// A new string with the replaced keys (tokens)
fn replace_tokens(source: &str, vars: &str, encode_as: &str) -> String {
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

///check_args is a very rudimentary cli opt check. It sees whether there are at least 4 arguments.
/// When there's less than 4 arguments, then the usage is printed and the application is exited with error code 1.
///
/// # Parameters:
///     args: is a Vec<Strings> with the cli arguments, obtained by the statement:
/// ```rust
/// env::args().collect();
/// ```
fn check_args(args: &Vec<String>) {
    if args.len() < 4 {
        println!(
            "usage: {} <source> <variables> <encode: html|txt> [dest]",
            args[0]
        );
        std::process::exit(1);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_open_file() {
        let result = open_file("template.txt");
        assert_eq!(result.len(), 102);
    }

    #[test]
    fn test_write_file() {
        let result = write_file("test.unittest", &String::from("Testing"));
        match result {
            Ok(()) => assert_eq!(1, 1),
            _ => assert_eq!(1, 0),
        }

        let result = open_file("test.unittest");
        assert_eq!(result, "Testing");

        let result = fs::remove_file("test.unittest");
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
