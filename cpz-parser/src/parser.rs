//! Using pest crate.
//! Parser module for cpluszero.

#[derive(Parser)]
#[grammar = "grammar.pest"]
/// Parser for cpluszero.
pub struct CpluszeroParser;
