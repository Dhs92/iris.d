use pest::Parser;
use pest_derive::*;

#[derive(Parser)]
#[grammar = "commands.pest"]
struct CommandParser;

fn main() {
    println!("{:#?}", CommandParser::parse(Rule::commands, "SQUIT localhost :Test").unwrap())
}
