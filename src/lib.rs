// #![feature(test)]
// extern crate test;

#[macro_use]
extern crate log;

#[macro_use]
#[feature(blas)]
extern crate ndarray;

// extern crate futures;
extern crate rand;

mod utils;
mod macros;

mod experiment;
pub use self::experiment::*;

pub mod agents;
pub mod domains;
pub mod fa;
pub mod geometry;
pub mod logging;
pub mod policies;
