use crate::error::Error;

pub mod posix_udp;

pub trait Communicate
{
    fn send<T: Send>(&self, data: T) -> Result<(), Error>;
    fn recv<T: Send>(&self) -> Result<T, Error>;
}
