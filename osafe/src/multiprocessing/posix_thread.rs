use core::{ffi, marker::PhantomData};

use alloc::{boxed::Box, string::{String, ToString}};

use crate::{error::{ErrNo, Error}, posix::{pthread_attr_t, pthread_cancel, pthread_create, pthread_join, pthread_t}};

use super::Joinable;
// create NULL constant
const NULL: *mut ffi::c_void = 0 as *mut ffi::c_void;

pub struct PosixThread<T>
{
    /// The POSIX thread handle
    handle: pthread_t,
    t: PhantomData<T>
}

/// This module provides functionality for creating and managing POSIX threads
/// for multiprocessing in Rust.
///
/// # Functions
///
/// - `thread_start`: An external C function that serves as the entry point for
///   a new thread. It takes a pointer to a `Job` and executes it, returning the
///   result as a pointer to `ffi::c_void`.
///
/// - `new`: A public function that creates a new thread to run the given `Job`.
///   It returns a `Result` containing either the new thread handle or an error.
///
/// # Errors
///
/// The `new` function can return an `Error::MultiProcessingErr` if the thread
/// creation fails. The error message is either the string representation of the
/// error code or a generic "Unknown error occurred" message.
///
/// # Safety
///
/// This module uses unsafe code to interface with the POSIX `pthread_create`
/// function and to manipulate raw pointers. Care must be taken to ensure that
/// pointers are valid and that memory is properly managed to avoid undefined
/// behavior.
impl<T> PosixThread<T>
{
    extern "C" fn thread_start<F>(args: *mut ffi::c_void) -> *mut ffi::c_void
    where F: FnOnce() -> T + Send + 'static
    {
        if args.is_null()
        {
            return NULL;
        }
        // Cast the arg to a Job pointer
        let job = args as *mut F;
        // Get the job from a raw pointer
        let job = unsafe{Box::from_raw(job)};
        // Execute the job and box the return value
        let ret = Box::new(Some(job()));
        // Return the pointer to the raw box
        return Box::into_raw(ret) as *mut ffi::c_void;
    }

    /// Creates a new Posix Thread
    pub fn new<F>(process: F) -> Result<Self, Error>
    where F: FnOnce() -> T + Send + 'static
    {
        // Allocate the handle
        let mut this = Self{
            handle: 0,
            t: PhantomData
        };
        // Box the job
        let job = Box::new(process);
        // Create the thread and get the status
        let status = unsafe{
            pthread_create(&mut this.handle as *mut pthread_t,
                0 as *const pthread_attr_t,
                Some(Self::thread_start::<F>),
                Box::into_raw(job) as *mut ffi::c_void
        )};
        // Check the status
        if status == 0
        {
            // Return OK since status was good
            return Ok(this);
        }
        return Err(Error::MultiProcessingErr(String::from_errno(status)))
    }
}

impl<T> Joinable<T> for PosixThread<T>
{
    fn join(&mut self) -> Result<Box<Option<T>>, Error> {
        let mut ret: *mut Option<T> = NULL as *mut Option<T>;
        let ret_ptr = &mut ret as *mut *mut Option<T>;
        let status = unsafe{
            pthread_join(self.handle, ret_ptr as *mut *mut ffi::c_void)
        };
        if status != 0
        {
            return Err(Error::MultiProcessingErr(String::from_errno(status)));
        }
        const PTHREAD_CANCELED: *mut ffi::c_void = usize::MAX as *mut ffi::c_void;
        if !ret.is_null() && ret as *mut ffi::c_void != PTHREAD_CANCELED
        {
            let ret = unsafe {
                Box::from_raw(ret)
            };
            self.handle = 0;
            return Ok(ret)
        }
        return Err(Error::MultiProcessingErr("Thread failed".to_string()));
    }
}

impl<T> Drop for PosixThread<T>
{
    fn drop(&mut self) {
        if self.handle != 0
        {
            unsafe
            {
                pthread_cancel(self.handle);
            }
            self.handle = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pthread()
    {
        let truth = 3;
        let modifier = 10;
        let mut starter = truth;
        let mut thread = PosixThread::new(move ||{
            starter += modifier;
            starter
        }).unwrap();
        let ret = thread.join().unwrap().unwrap();
        assert_eq!(ret, truth + modifier);
    }
    #[test]
    fn test_pthread_cancel()
    {
        let truth = 3;
        let modifier = 1;
        let mut starter = truth;
        {
            let thread = PosixThread::new(move ||{
                for _ in 0..1E9 as i64
                {
                    starter += modifier;
                }
                starter
            }).unwrap();
            assert_ne!(thread.handle, 0);
        }
        let test = 2;
        assert_eq!(test, 2);
    }
}
