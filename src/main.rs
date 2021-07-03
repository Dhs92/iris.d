use pest::Parser;
use pest_derive::*;

#[derive(Parser)]
#[grammar = "commands.pest"]
struct CommandParser;

fn main() {
    println!("{:#?}", CommandParser::parse(Rule::commands, "PRIVMSG Yuuki,#foobar,GH0S1,yuuki@local.host :Fuk u!").unwrap());
//     ('\u{01}'..='\u{09}').for_each(|c| print!("{}", c));
//     println!(); 
//     ('\u{0B}'..='\u{0C}').for_each(|c| print!("{}", c));
//     println!();
//     ('\u{0E}'..='\u{1F}').for_each(|c| print!("{}", c));
//     println!();
//     ('\u{21}'..='\u{3F}').for_each(|c| print!("{}", c));
//     println!();
//     ('\u{41}'..='\u{FF}').for_each(|c| print!("{}", c));
//     println!();
}