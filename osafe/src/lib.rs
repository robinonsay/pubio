#![no_std]

extern crate alloc;

mod posix
{
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(dead_code)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub mod error;
pub mod io;
pub mod multiprocessing;
pub mod ipc;
