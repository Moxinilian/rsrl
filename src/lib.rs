#[macro_use] extern crate slog;
extern crate slog_term;
extern crate slog_async;

extern crate blas;
extern crate openblas_src;

#[macro_use] extern crate ndarray;

extern crate serde;
extern crate serde_json;
extern crate serde_test;
#[macro_use] extern crate serde_derive;

// extern crate futures;
extern crate rand;

mod utils;
mod consts;
mod macros;

mod experiment;
pub use self::experiment::*;

mod parameter;
pub use self::parameter::Parameter;

pub mod agents;
pub mod domains;
pub mod fa;
pub mod geometry;
pub mod logging;
pub mod policies;
