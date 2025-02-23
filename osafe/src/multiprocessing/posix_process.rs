use alloc::{format, string::String};

use crate::{error::{ErrNo, Error}, posix::{__errno_location, fork, kill, pid_t}};

pub enum Signal
{
    Kill,
    Interrupt
}

pub struct PosixProcess
{
    pid: pid_t
}

impl PosixProcess
{
    pub fn fork<F>(entry: F) -> Result<Option<Self>, Error>
    where F: FnOnce() + Send + 'static
    {
        let mut process = Self
        {
            pid: 0
        };
        process.pid = unsafe{fork()};
        if process.pid == 0
        {
            entry();
            return Ok(None);
        }
        else if process.pid == -1
        {
            let errno = unsafe{* __errno_location()};
            return Err(Error::MultiProcessingErr(String::from_errno(errno)))
        }
        else
        {
            return Ok(Some(process));
        }
    }

    pub fn signal(self, sig: Signal) -> Result<(), Error>
    {
        let sig = match sig
        {
            Signal::Kill => 9,      // SIGKILL
            Signal::Interrupt => 2, // SIGINT
        };
        let ret = unsafe{kill(self.pid, sig)};
        if ret != 0
        {
            return Err(Error::MultiProcessingErr(format!("Failed to send signal {}", sig)))
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{io::{posix_print::PosixPrint, Printable}, posix::sleep};

    #[test]
    fn test_process()
    {
        let process = PosixProcess::fork(||{
            PosixPrint::println("Hello from a new process!").unwrap();
            unsafe { sleep(60) };
        }).unwrap();
        match process {
            Some(process) =>
            {
                PosixPrint::printstrln(&format!("The new process is {}", process.pid)).unwrap();
                unsafe { sleep(1) };
                process.signal(Signal::Interrupt).unwrap();
            },
            None => return,
        }
    }
}