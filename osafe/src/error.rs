use alloc::string::String;

#[derive(Debug)]
pub enum Error
{
    IoErr(String)
}