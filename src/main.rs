extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "types.pest"]
struct TypesParser{
  first_field: u16,
  second_field: u8,
  third_field: bool
}

fn main() {
  // macro for defining and filling structs
  // macro_rules! create_struct {
  //   ($name: ident, $($i: ident, $e: expr),*) => {{
  //     struct $name {
  //       $($i: u16),*
  //     }

  //     let tmp: $name = $name {
  //       $($i: $e),*
  //     };

  //     tmp
  //   }}
  // }

  let t = TypesParser::parse_and_create(Rule::t, &[9, 0, 1, 0], true);
  println!("first_field: {}\nsecond_field: {}\nthird_field: {}", t.first_field, t.second_field, t.third_field);
}
