extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

// #[derive(Parser)]
// struct Child {
//   first_field: u32
// }

#[derive(Parser)]
struct Child {
  first_field: u16,
  second_field: bool,
  third_field: bool
}

#[derive(Parser)]
struct TypesParser {
  first_field: Child,
  second_field: u8
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

  let input = &[9, 0, 1, 0, 8];
  let t = TypesParser::parse_and_create(true, &mut input.to_vec());
}
