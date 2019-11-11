#![no_std]
use contract_sdk::{
    ink_lang::contract
};

contract! {
  #![env = DefaultSrmlTypes]
  // Define contract data
  struct Demo {}

  // Define contract functions
  impl Demo {}

  // Define contract instantiation logic
  impl Deploy for Demo {
    fn deploy(&mut self) {}
  }
}
