#[macro_use]
extern crate failure;

// #[macro_use]
// extern crate lazy_static;

#[macro_use]
extern crate serde;

#[macro_use]
extern crate derive_new;

#[cfg(test)]
#[macro_use]
extern crate serde_json;

#[macro_use]
mod macros;

pub mod core;
pub mod json;

pub mod prelude;

// pub mod core;
// pub mod json;
// pub mod matchers;
