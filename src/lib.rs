// ownership , lifetimes

// extern crate clap;
use clap::{App, Arg, ArgMatches};

pub struct Operands {
      pub first_operand: i64,
      pub second_operand: i64,
      pub operator: String,
}
impl Operands {
      fn new(first_operand: i64, second_operand: i64, operator: String) -> Operands {
            Operands {
                  first_operand,
                  second_operand,
                  operator,
            }
      }
}

pub fn initialize() -> ArgMatches<'static> {
      App::new("My Calculator Program")
            .version("1.0")
            .author("Rishabh Raghwendra")
            .about("Does awesome things")
            .arg(Arg::with_name("add")
                  .short("a")
                  .long("add")
                  .help("Adds two values")
                  .takes_value(true)
                  .value_names(&["first", "second"]))
            .arg(Arg::with_name("substract")
                  .short("s")
                  .long("sub")
                  .help("Substract two values")
                  .takes_value(true)
                  .value_names(&["first", "second"]))
            .arg(Arg::with_name("multiply")
                  .short("m")
                  .long("mult")
                  .help("Multiply two values")
                  .takes_value(true)
                  .value_names(&["first", "second"]))
            .arg(Arg::with_name("divide")
                  .short("d")
                  .long("divide")
                  .help("Divide two values")
                  .takes_value(true)
                  .value_names(&["first", "second"]))
            .get_matches()
}
pub fn take_user_input()-> Operands{
      let app = initialize();
      let operator = app.args.keys().copied().next().unwrap();
      let first_operand = &app.args[operator].vals[0].clone().into_string().expect("Error");
      let first_operand:i64 = first_operand.parse().expect("error in parsing");
      let second_operand: &String= &app.args[operator].vals[1].clone().into_string().expect("Error");
      let second_operand:i64 = second_operand.parse().expect("error in parsing");
      Operands::new(first_operand, second_operand, operator.to_string())
}
 
fn sum(first_operand: i64, second_operand: i64) -> i64 {
      first_operand + second_operand
}
fn sub(first_operand: i64, second_operand: i64) -> i64 {
      first_operand - second_operand
}
fn mult(first_operand: i64, second_operand: i64) -> i64 {
      first_operand * second_operand
}
fn divide(first_operand: i64, second_operand: i64) -> i64 {
      if second_operand != 0 {
            return first_operand / second_operand;
      }
      assert!(false);
      6
}

pub fn calculate(operators: &Operands) {
      if operators.operator == "add" {
            display(sum(operators.first_operand, operators.second_operand));
      } else if operators.operator == "substract" {
            display(sub(operators.first_operand, operators.second_operand));
      } else if operators.operator == "multiply" {
            display(mult(operators.first_operand, operators.second_operand));
      } else {
            display(divide(operators.first_operand, operators.second_operand));
      }
}

fn display(result: i64) {
      println!("{}", result);
}
