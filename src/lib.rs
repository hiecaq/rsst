//! # rsst
//!
//! A command line tool that dumps RSS feeds into files.

#![warn(clippy::pedantic)]
#![warn(clippy::clone_on_ref_ptr)]
#![warn(clippy::decimal_literal_representation)]
#![allow(clippy::cast_possible_truncation)]

pub mod cli;
pub mod config;
pub mod downstream;
pub mod metadata;
pub mod upstream;
pub mod util;
