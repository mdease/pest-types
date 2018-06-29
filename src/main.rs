extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "types.pest"]
struct TypesParser;

fn main() {
  // macro for defining and filling structs
  macro_rules! create_struct {
    ($name: ident, $($i: ident, $e: expr),*) => {{
      struct $name {
        $($i: u16),*
      }

      let tmp: $name = $name {
        $($i: $e),*
      };

      tmp
    }}
  }

  // bytes for 9 and 5 as u16
  let pairs = TypesParser::parse(Rule::t, &[9, 0, 5, 0]).unwrap_or_else(|e| panic!("{}", e));

  for pair in pairs {
    // get the matched u16s
    let nums = pair.clone().into_span().as_type();

    // create the struct
    let my_struct = create_struct!(MyNums, number_one, nums[0], number_two, nums[1]);

    // check struct fields
    println!("MyNums.number_one: {}", my_struct.number_one);
    println!("MyNums.number_two: {}", my_struct.number_two);
  }
}
