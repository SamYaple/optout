extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "opts.pest"]
struct OptParser;

fn main() {
    let pairs = OptParser::parse(Rule::options, " --debug -d --help -h --wut nothing else parses --debug").unwrap_or_else(|e| panic!("{}", e));

    // Because ident_list is silent, the iterator will contain idents
    for pair in pairs {
        // A pair is a combination of the rule which matched and a span of input
        //println!("Rule:    {:?}", pair.as_rule());
        //println!("Span:    {:?}", pair.as_span());
        //println!("Text:    {}", pair.as_str());

        // A pair can be converted to an iterator of the tokens which make it up:
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::debug => println!("Rule: {:?}", inner_pair.as_rule()),
                Rule::help  => println!("Rule: {:?}",  inner_pair.as_rule()),
                _ => unreachable!()
            };
        }
    }
}
