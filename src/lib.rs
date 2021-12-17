// ownership , lifetimes


use std::env;
// extern crate clap;
// use clap::{App, Arg, ArgMatches};

pub struct Operands {
      pub first_operand: u64,
      pub second_operand: u64,
      pub operator: String,
}
impl Operands {
      fn new(first_operand: u64, second_operand: u64, operator: String) -> Operands {
            Operands {
                  first_operand,
                  second_operand,
                  operator,
            }
      }
}

pub fn take_user_input() -> Operands {
      let args: Vec<String> = env::args().collect();
      let first_operand: u64 = args[1].parse().unwrap();
      let second_operand: u64 = args[2].parse().unwrap();
      let operator: String = args[3].to_string();
      Operands::new(first_operand, second_operand, operator)
}

fn sum(first_operand: u64, second_operand: u64) -> u64 {
      first_operand + second_operand
}
fn sub(first_operand: u64, second_operand: u64) -> u64 {
      first_operand - second_operand
}
fn mult(first_operand: u64, second_operand: u64) -> u64 {
      first_operand * second_operand
}
fn divide(first_operand: u64, second_operand: u64) -> u64 {
      if second_operand != 0 {
            return first_operand / second_operand;
      } 
      assert!(false);
      6
}

pub fn calculate(operators: &Operands) {
      if operators.operator == "add" {
            display(sum(operators.first_operand, operators.second_operand));
      } else if operators.operator == "sub" {
            display(sub(operators.first_operand, operators.second_operand));
      } else if operators.operator == "mult" {
            display(mult(operators.first_operand, operators.second_operand));
      } else {
            display(divide(operators.first_operand, operators.second_operand));
      }
}

fn display(result: u64) {
      println!("{}", result);
}
