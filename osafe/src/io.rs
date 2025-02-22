use alloc::string::String;
use super::error::Error;
pub mod posix;

pub trait Printable
{
    fn print(msg: &str) -> Result<usize, Error>;
    fn println(msg: &str) -> Result<usize, Error>;
    fn printstr(msg: &String) -> Result<usize, Error>;
    fn printstrln(msg: &String) -> Result<usize, Error>;
}
