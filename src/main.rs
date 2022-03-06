extern crate html_escape;
use replacer::*;
use std::{env, process};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if !check_args(&args) {
        process::exit(1);
    }

    let source = read_file_or_exit(&args[1]);
    let vars = read_file_or_exit(&args[2]);
    let encode_as = &args[3];

    let replaced = replace_tokens(&source, &vars, &encode_as.to_string());

    println!("\n-------------\n{}\n-------------\n", replaced);

    if args.len() == 4 {
        return write_file(&args[1], &replaced);
    } else {
        return write_file(&args[4], &replaced);
    }
}
