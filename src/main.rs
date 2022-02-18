use std::env;
use std::fs::{self, File};
use std::io::Write;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    check_args(&args);
    let source = open_file(&args[1]);
    let vars = open_file(&args[2]);
    let replaced = replace_tokes(source, vars);

    println!("{}", replaced);
    if args.len() == 3 {
        return write_file(&args[1], &replaced);
    } else {
        return write_file(&args[3], &replaced);
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

    println!("File not found: {}", file);
    std::process::exit(1);
}

fn replace_tokes(source: String, vars: String) -> String {
    let mut result: String;
    result = source.clone();

    let json: serde_json::Value = serde_json::from_str(&vars).expect("JSON malformed");
    for (key, _value) in json["vars"].as_object().unwrap() {
        result = result.replace(key, json["vars"][key].as_str().unwrap());
    }
    result
}

fn check_args(args: &Vec<String>) {
    if args.len() < 3 {
        println!("usage replaced <source> <variables> <dest>");
        std::process::exit(1);
    }
}
