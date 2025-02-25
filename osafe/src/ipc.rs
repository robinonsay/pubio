use crate::error::Error;

pub mod posix_udp;

pub trait Communicate
{
    fn send<T: Send>(&self, data: T) -> Result<(), Error>;
    /// Attempts to recv data for time in.
    /// If timeout is negative it will block indefinetly.
    /// If timeout is 0 it will return immediatley
    fn try_send<T: Send>(&self, data: T, timeout_ms:i32) -> Result<(), Error>;
    fn recv<T: Send>(&self) -> Result<T, Error>;
    /// Attempts to recv data for time in.
    /// If timeout is negative it will block indefinetly.
    /// If timeout is 0 it will return immediatley
    fn try_recv<T: Send>(&self, timeout_ms: i32) -> Result<Option<T>, Error>;
}
