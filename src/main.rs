extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "types.pest"]
struct TypesParser;

fn main() {
  // bytes for 9 as a u16
  let pairs = TypesParser::parse(Rule::t, &[9, 0]).unwrap_or_else(|e| panic!("{}", e));

  for pair in pairs {
  	let span = pair.clone().into_span();
  	let matched: u16 = span.as_type();

  	println!("Rule:    {:?}", pair.as_rule());
  	println!("Matched:    {}", matched);
  }
}
