use alloc::string::String;
pub mod posix;

pub trait Printable
{
    fn print(msg: &str) -> Result<usize, ()>;
    fn println(msg: &str) -> Result<usize, ()>;
    fn printstr(msg: &String) -> Result<usize, ()>;
    fn printstrln(msg: &String) -> Result<usize, ()>;
}
