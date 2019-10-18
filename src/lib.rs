#[macro_use]
extern crate failure;

// #[macro_use]
// extern crate lazy_static;

#[macro_use]
extern crate serde;

#[cfg(test)]
#[macro_use]
extern crate serde_json;

#[macro_use]
mod macros;

pub mod core;
pub mod json;

// pub mod core;
// pub mod json;
// pub mod matchers;
