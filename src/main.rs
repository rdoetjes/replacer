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
    let replaced = replace_tokes(&source, &vars, &encode_as.to_string());

    println!("{}", replaced);

    if args.len() == 4 {
        return write_file(&args[1], &replaced);
    } else {
        return write_file(&args[4], &replaced);
    }
}

fn write_file(file: &str, data: &String) -> std::io::Result<()> {
    let mut file = File::create(file)?;
    file.write_all(data.as_bytes())?;
    file.sync_all()?;
    Ok(())
}

fn open_file(file: &str) -> String {
    let result: String;

    if std::path::Path::new(&file).exists() {
        result = fs::read_to_string(file.to_string()).expect("Could not open file");
        return result;
    }

    eprintln!("File not found: {}", file);
    std::process::exit(1);
}

fn replace_tokes(source: &String, vars: &String, encode_as: &String) -> String {
    let mut result: String;
    result = source.clone();

    let json: serde_json::Value = serde_json::from_str(&vars).expect("JSON malformed");

    if let Some(field) = json.get("vars") {
        for (key, _value) in field.as_object().unwrap() {
            match encode_as.as_str() {
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
        let json = open_file("vars.json");
        let template = open_file("template.txt");

        let mut result = replace_tokes(&template, &json, &String::from("txt"));
        assert_eq!(result.contains("Where you see D it should say D"), true);
        assert_eq!(
            result.contains("Where we see <this> it should say this"),
            true
        );

        result = replace_tokes(&template, &json, &String::from("html"));
        assert_eq!(result.contains(r#"&lt;this&gt;"#), true);
    }
}
