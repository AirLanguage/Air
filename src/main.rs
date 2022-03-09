mod cmd;
mod lexer;
mod tests;

use clap::{Arg, Command};
// use std::fs::read_to_string;

const VERSION: &str = "0.1-beta";

fn main() {
    #[allow(unused_variables)]
    let matches = Command::new("Air")
        .version(VERSION)
        .author("Mateusz Słotwiński <mateusz@mslotwinski.eu>")
        .about("The official interpreter for Air language.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("run")
                .about("Run a Air file.")
                .version(VERSION)
                .arg(
                    Arg::new("file")
                        .help("The Air file to execute.")
                        .required(true),
                ),
        )
        .get_matches();

    // if let Some(ref run) = matches.subcommand_matches("run") {
    //     let file = run.value_of("file").unwrap();
    //     let path = std::path::PathBuf::from(file);
    //     let contents = read_to_string(file).unwrap();
    //     let tokens = generate(contents.as_str());

    //     match parse(tokens) {
    //         Ok(ast) => {
    //             match interpret(ast, path) {
    //                 Ok(_) => {}
    //                 Err(e) => e.print(),
    //             };
    //         }
    //         Err(e) => e.print(),
    //     };
    // } else if let Some(ref js) = matches.subcommand_matches("js") {
    //     let file = js.value_of("file").unwrap();
    //     let contents = read_to_string(file).unwrap();
    //     let output = js.value_of("output").unwrap();
    //     let tokens = generate(contents.as_str());

    //     match parse(tokens) {
    //         Ok(ast) => {
    //             match cmd::js(ast, output) {
    //                 Ok(_) => {}
    //                 Err(e) => e.print(),
    //             };
    //         }
    //         Err(e) => e.print(),
    //     };
    // }
}
