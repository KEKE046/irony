#![allow(unused)]


/// experimental mod includes an example for the expanded target
/// TODO: turn into circt binding
mod experimental;


/// ## Infra
/// traits and structs for IR construction
/// 
/// - [x] Entity
/// - [x] Operation
/// - [x] Constraint
/// - [ ] Environ
/// - [ ] Parse & Print
/// - [ ] ...

mod infra;


pub use infra::*;
pub use irony_macros::*;
