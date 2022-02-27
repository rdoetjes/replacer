extern crate html_escape;
use replacer::*;
use std::fs;
use std::{env, process};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if !check_args(&args) {
        process::exit(1);
    }

    let file: &str = &args[1];
    let source = read_file_or_exit(file);

    let file: &str = &args[2];
    let vars = read_file_or_exit(file);

    let encode_as = &args[3];
    let replaced = replace_tokens(&source, &vars, &encode_as.to_string());

    println!("{}", replaced);

    if args.len() == 4 {
        return write_file(&args[1], &replaced);
    } else {
        return write_file(&args[4], &replaced);
    }
}

// Tries to open and read the data from the file
// When it fails, it will write an error to stderr and exits the application
//
// # Paramaters:
// file: the file name to open and read from
//
// #Return:
// the contents of the file when succeeds
fn read_file_or_exit(file: &str) -> String {
    let result = fs::read_to_string(file.to_string());

    let contents = match result {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("Could not open file {}\nError: {}", file, e);
            process::exit(1)
        }
    };

    contents
}

//check_args is a very rudimentary cli opt check. It sees whether there are at least 4 arguments.
// When there's less than 4 arguments, then the usage is printed and the application is exited with error code 1.
//
// # Parameters:
//     args: is a Vec<Strings> with the cli arguments, obtained by the statement:
// ```rust
// env::args().collect();
// ```
fn check_args(args: &Vec<String>) -> bool {
    if args.len() < 4 {
        println!(
            "usage: {} <source> <variables> <encode: html|txt> [dest]",
            args[0]
        );
        return false;
    }
    return true;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_check_args() {
        let mut validate = Vec::from([String::from("one"), String::from("two")]);
        assert_eq!(check_args(&validate), false);
        validate.push(String::from("three"));
        assert_eq!(check_args(&validate), false);
        validate.push(String::from("four"));
        assert_eq!(check_args(&validate), true);
        validate.push(String::from("five"));
        assert_eq!(check_args(&validate), true);
    }

    #[test]
    fn test_read_file_or_exit() {
        assert_eq!(read_file_or_exit("template.txt").len(), 102);
    }
}
