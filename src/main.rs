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
    let args = &raw_args.join(" ");

    let pairs = OptParser::parse(Rule::options, args)?;
    for pair in pairs {
        match pair.as_rule() {
            Rule::debug => println!("Debug is set"),
            Rule::help  => help(),
            Rule::prog  => {
                let path = std::path::Path::new(pair.into_inner().as_str());
                println!("Program: {}", path.display());
            }
            Rule::file  => {
                let path = std::path::Path::new(pair.into_inner().as_str());
                println!("File: {}", path.display());
            },
            Rule::kvlist => {
                for rule in pair.into_inner() {
                    println!("Rule: {:?}", rule);
                    println!("kvpair found: {}", rule.as_str());
                }
            },
            Rule::EOI   => (),
            _ => unreachable!(),
        };
    }
    Ok(())
}
