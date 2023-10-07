//#![allow(unused)]
//#![allow(unused_variables)]
//#![cfg_attr(nightly, feature(implied_bounds))]
//#![cfg_attr(nightly, feature(lazy_type_alias))]
//#![cfg_attr(nightly, feature(min_specialization))]
//! Any `S` generic parameter is for [String]/[str] slice-like type, used for accepting names of
//! directories, files/binary crates, features...
//!
//! Any `B` generic parameter is for [BinaryCrateName]. That's separate from `S` because of
//! lifetimes and borrowing.
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod group;
pub mod group_of_sequences_of_groups;
pub mod indicators;
pub mod output;
mod run;
pub mod sequence_of_groups;
mod task;
#[cfg(test)]
mod unit_tests;
