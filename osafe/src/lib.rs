#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate alloc;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub mod error;
pub mod io;
pub mod multiprocessing;
