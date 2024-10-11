#![allow(dead_code)]

use std::{collections::HashMap, io, fmt, error::Error};
use error_stack::{IntoReport, Report, Result, ResultExt};

#[derive(Debug)]
struct ParsePayInfoErr;

impl fmt::Display for ParsePayInfoErr{
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result{
    fmt.write_str("Parsing payment err: invalid payment info")
  }
}

impl Error for ParsePayInfoErr{}

fn parse_card_nums(card: &str) -> Result<Vec<u32>, ParsePayInfoErr>{
  let nums = card 
        .split(" ")
        .into_iter()
        .map(|s|{
          s.parse()
          .report()
          .attach_printable_lazy(||{
            format!("{s:?} could not be parsed as u32")
          })
        })
        .collect::<Result<Vec<u32>, _>>()
        .change_context(ParsePayInfoErr)
        .attach_printable(format!(
          "Failed to parse input as a num, input: {card}"
        ))?;

    Ok(nums)
}

#[derive(Debug)]
struct Exp{
  yr: u32,
  mon: u32
}

#[derive(Debug)]
struct Card{
  num: u32,
  exp: Exp,
  cvv: u32
}

fn parse_card(card: &str) -> Result<Card, ParsePayInfoErr>{
  let mut nums = parse_card_nums(card)?;
  
  let len = nums.len();
  let exp_len = 4;

  if len !=  exp_len{
    return Err(Report::new(ParsePayInfoErr)
      .attach_printable(format!(
          "Incorrect num of ele parsed. Expd {exp_len} but got {len}, ele: {nums:?}"
    )))
  }

  let cvv = nums.pop().unwrap();
  let yr = nums.pop().unwrap();
  let mon = nums.pop().unwrap();
  let num = nums.pop().unwrap();

  Ok(Card{
    num,
    exp: Exp{yr, mon},
    cvv
  })
}

#[derive(Debug)]
enum CardErr{
  InvalidInp(String),
  Other
}

impl fmt::Display for CardErr{
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result{
    fmt.write_str("Card Err: could not retrive card")
  }
}

impl Error for CardErr{}

fn get_card_info(
  credit_card: &HashMap<&str, &str>,
  name: &str,
) -> Result<Card, CardErr>{
  let card_str = credit_card.get(name).ok_or_else(||{
    let msg = format!("no card found for the name {name}");
    Report::new(CardErr::InvalidInp(msg.clone()))
      .attach_printable(msg.clone())
  })?;

  let card = parse_card(card_str)
    .change_context(CardErr::Other)
    .attach_printable(format!("{name}'s card could not be parsed"))?;

  Ok(card)
}


fn main() {
  env_logger::init();

  let cards = HashMap::from([
    ("raiden", "1122334 10 35 789"),
    ("katana", "2211443 08 28 123"),
    ("lui", "4455667 11 22 456"),
  ]);

  let mut name = String::new();

  println!("enter name: ");
  io::stdin() 
    .read_line(&mut name) 
    .expect("give correct name");

  let res = get_card_info(&cards, name.trim());

  match res{
    Ok(card) => {
      println!("\nCard Info: {card:?}");
    },
    Err(err) => {
      match err.current_context(){
        CardErr::InvalidInp(msg) => println!("\n{msg}"),
        CardErr::Other => println!("\nsomething went wrong")
      }

      log::error!("\n{err:?}");
    }
  }
}