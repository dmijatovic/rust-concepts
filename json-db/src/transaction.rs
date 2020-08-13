
// use std::result::Result;
use serde_derive::{Serialize,Deserialize};

// create new vector
// let mut db = Vec::new();

#[derive(Debug,Serialize,Deserialize)]
pub struct Transaction{
  pub from: String,
  pub to:String,
  pub amount: u64
}

pub fn get_transactions()->Result<Vec<Transaction>, String>{
  // database file
  let file_name = "db/transaction.json".to_string();
  // just a basic unwrap without handling error
  // let f = std::fs::read_to_string(file_name).unwrap();

  // handle error
  let f = match std::fs::read_to_string(file_name){
    Ok(r)=> r,
    Err(e)=> return Err(e.to_string()),
  };

  let db:Vec<Transaction> = match serde_json::from_str(&f){
    Ok(json)=>json,
    Err(e)=> return Err(e.to_string())
  };

  Ok(db)
}


/* Using methods defined on Result enum
  see https://doc.rust-lang.org/std/result/enum.Result.html
  for all available methods
*/
pub fn get_transactions_combinators()->Result<Vec<Transaction>,String>{
  // database file
  let file_name = "db/transaction.json".to_string();

  let f = std::fs::read_to_string(file_name);
  f.map_err(|e|e.to_string())
    .and_then(|d| {
      serde_json::from_str(&d).map_err(|e|e.to_string())
    })
}

/**
 * Using custom error. It can be enum or struct.
 * Defines different types of error we can expect.
 * In this case io::Error and serde_json::Error
 * We need to implement From trait for error conversion
 */
 #[derive(Debug)]
pub enum TransactionError{
  LoadError(std::io::Error),
  ParseError(serde_json::Error),
}

impl TransactionError{
  pub fn to_string(&self) -> String{
    // println!("TE, self: {:?}", *self);
    match &self{
      TransactionError::LoadError(e)=>{e.to_string()},
      TransactionError::ParseError(e)=>{e.to_string()},
    }
    // "Custom error".to_string()
  }
}

impl From<std::io::Error> for TransactionError{
  fn from(e: std::io::Error) ->Self{
    TransactionError::LoadError(e)
  }
}

impl From<serde_json::Error> for TransactionError{
  fn from(e: serde_json::Error) ->Self{
    TransactionError::ParseError(e)
  }
}

pub fn get_transactions_custom()->Result<Vec<Transaction>,TransactionError>{
  // database file
  let file_name = "db/transaction.json".to_string();

  // use of ? it is unwrap macro that will
  // parse error into custom enum if from is
  // implemented (we did that see lines 64+)
  let f = std::fs::read_to_string(file_name)?;
  Ok(serde_json::from_str(&f)?)
}
