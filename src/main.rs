extern crate html_escape;
use replacer::*;
use std::{env, process};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if !check_args(&args) {
        process::exit(1);
    }

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
    use crate::check_args;

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
}
