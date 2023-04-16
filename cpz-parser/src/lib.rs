//! parser crate for cpluszero.
//! using pest crate

extern crate pest;
#[macro_use]
extern crate pest_derive;

pub use parser::*;

mod parser;
pub mod ast;

