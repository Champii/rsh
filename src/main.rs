#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![warn(clippy::cargo)]
#![deny(clippy::restriction)]
//
#![allow(clippy::module_name_repetitions)]
//
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::implicit_return)]
#![allow(clippy::integer_arithmetic)]
#![allow(clippy::float_arithmetic)]
#![allow(clippy::integer_division)]
#![allow(clippy::match_wild_err_arm)]
#![allow(clippy::match_same_arms)] //tmp
#![warn(clippy::print_stdout)] //tmp
#![warn(clippy::use_debug)] //tmp
#![allow(clippy::indexing_slicing)] //false positive on macro attrs
#![deny(unused_must_use)]

#[macro_use]
extern crate quick_error;

mod error;
mod exec;
mod input;
mod parsing;
mod rsh;

use error::Error;

fn main() -> Result<(), Error> {
    rsh::run()
}
