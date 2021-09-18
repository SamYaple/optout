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
    println!("{:?}", raw_args);
    println!("{}", args);

    let pairs = OptParser::parse(Rule::options, args)?;
    //println!("{:?}", pairs);
    for pair in pairs {
        match pair.as_rule() {
            Rule::prog  => println!("Program: {}", pair.into_inner().as_str()),
            Rule::debug => println!("Debug is set"),
            Rule::file  => {
                let path = pair.into_inner().as_str();
                println!("File path: {}", path)
            },
            Rule::help  => help(),
            Rule::kvlist => {
                for rule in pair.into_inner() {
                    println!("kvpair found: {}", rule.as_str());
                }
            },
            Rule::EOI   => (),
            _ => unreachable!(),
        };
    }
    Ok(())
}
