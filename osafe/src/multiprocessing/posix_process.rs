use core::ffi;

use alloc::{format, string::String};

use crate::{error::{ErrNo, Error}, io::{posix_print::Print, Printable}, posix::{__errno_location, __sigset_t, fork, kill, pid_t, sigaction, sigaction__bindgen_ty_1, SIGINT, SIGKILL, SIGTERM}};


#[allow(dead_code)]
enum Signal
{
    Kill,
    Interrupt,
    Term,
}

pub struct Process
{
    pid: pid_t
}

static mut ON_EXIT: Option<fn()> = None;

impl Process
{
    extern "C" fn signal_handler(_: ffi::c_int)
    {
        unsafe
        {

            match ON_EXIT
            {
                Some(on_exit) => on_exit(),
                None => return
            }
        }
    }

    pub fn register_exit_hdlr(hdlr: fn()) -> Result<(), Error>
    {
        unsafe
        {
            ON_EXIT = Some(hdlr);
        }
        let saction = sigaction{
            __sigaction_handler: sigaction__bindgen_ty_1{
                sa_handler: Some(Process::signal_handler)
            },
            sa_mask: __sigset_t{__val: [0; 16]},
            sa_flags: 0,
            sa_restorer: None
        };
        let ret = unsafe{sigaction(SIGINT as i32, &saction as *const sigaction, 0 as *mut sigaction)};
        if ret == -1
        {
            let errno = unsafe {
                *__errno_location()
            };
            return Err(Error::MultiProcessingErr(String::from_errno(errno)));
        }
        let ret = unsafe{sigaction(SIGTERM as i32, &saction as *const sigaction, 0 as *mut sigaction)};
        if ret == -1
        {
            let errno = unsafe {
                *__errno_location()
            };
            return Err(Error::MultiProcessingErr(String::from_errno(errno)));
        }
        Ok(())
    }

    pub fn run<F>(entry: F) -> Result<Option<Self>, Error>
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

    fn signal(&self, sig: Signal) -> Result<(), Error>
    {
        let sig = match sig
        {
            Signal::Kill => SIGKILL,      // SIGKILL
            Signal::Interrupt => SIGINT, // SIGINT
            Signal::Term => SIGTERM,     // SIGTERM
        };
        if self.pid > 0
        {
            let ret = unsafe{kill(self.pid, sig as i32)};
            if ret != 0
            {
                return Err(Error::MultiProcessingErr(format!("Failed to send signal {}", sig)))
            }
            return Ok(())
        }
        Ok(())
    }
}

impl Drop for Process
{
    fn drop(&mut self) {
        if self.pid > 0
        {
            Print::printstrln(&format!("Killing process {}", self.pid)).unwrap();
            self.signal(Signal::Interrupt).unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{io::{posix_print::Print, Printable}, posix::sleep};

    #[test]
    fn test_process()
    {
        let process = Process::run(||{
            Print::println("Hello from a new process!").unwrap();
            unsafe { sleep(10) };
            Print::println("Process is still running!").unwrap();
        }).unwrap();
        match process {
            Some(process) =>
            {
                Print::printstrln(&format!("The new process is {}", process.pid)).unwrap();
                unsafe { sleep(1) };
                process.signal(Signal::Interrupt).unwrap();
            },
            None => return,
        }
    }
    #[test]
    fn test_process_drop()
    {
        {
            let _process = Process::run(||{
                Print::println("Hello from a new process!").unwrap();
                unsafe { sleep(5) };
                Print::println("Process is still running!").unwrap();
            }).unwrap();
            unsafe { sleep(1) };
        }
    }
}