#![no_std]
use ink_lang::contract;

contract! {
  // Define contract data
  struct Demo {}

  // Define contract functions
  impl Demo {}

  // Define contract instantiation logic
  impl Deploy for Demo {
    fn deploy(&mut self) {}
  }
}
