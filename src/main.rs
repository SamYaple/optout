extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "opts.pest"]
struct OptParser;

fn help() {
    println!("Showing Help");
}

fn main() -> Result<(), pest::error::Error<Rule>> {
    let raw_args: Vec<String> = std::env::args().collect();
    // Drop the 0th element which is the program name in common situations
    let args = &raw_args[1..raw_args.len()].join(" ");

    let pairs = OptParser::parse(Rule::options, args)?;
    for pair in pairs {
        match pair.as_rule() {
            Rule::debug => println!("Debug is set"),
            Rule::file  => {
                let path = pair.into_inner().as_str();
                println!("File path: {:?}", path)
            },
            Rule::help  => help(),
            _ => help(),
        };
    }
    Ok(())
}
