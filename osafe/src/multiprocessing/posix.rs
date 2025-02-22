use core::{char::MAX, ffi};

use alloc::{boxed::Box, fmt::format, string::{String, ToString}};

use crate::{error::Error, pthread_attr_t, pthread_create, pthread_t, strerror, strlen, strnlen};

type Job<T> = fn() -> T;

struct PosixThread<T>
{
    /// The POSIX thread handle
    handle: pthread_t,
    /// The process to run in the thread
    process: Job<T>,
    ret: Option<T>
}

impl<T> PosixThread<T>
{
    extern "C" fn thread_start(args: *mut ffi::c_void) -> *mut ffi::c_void
    {
        const NULL: *mut ffi::c_void = 0 as *mut ffi::c_void;
        if args.is_null()
        {
            return NULL;
        }
        let this = args as *mut PosixThread<T>;
        let this = match unsafe{this.as_mut()}
        {
            Some(this) => this,
            None => return NULL
        };
        let ret = (this.process)();
        this.ret = Some(ret);
        return NULL;
    }

    pub fn new<F>(process: Job<T>) -> Result<Self, Error>
    {
        let mut this = Self{
            handle: 0,
            process: process,
            ret: None
        };
        let ret = unsafe{
            pthread_create(&mut this.handle as *mut pthread_t,
                0 as *const pthread_attr_t,
                Some(Self::thread_start),
                &this as *const _ as *mut ffi::c_void
        )};
        if ret == 0
        {
            return Ok(this);
        }
        else
        {

            let err_msg = unsafe{strerror(ret)} as *mut char;
            if !err_msg.is_null()
            {
                const MAX_LEN:usize = 256;
                let mut err_buff = [0 as char; MAX_LEN];
                unsafe{err_msg.copy_to(err_buff.as_mut_ptr(), err_buff.len())};
                let mut err_str = String::new();
                for c in err_buff
                {
                    if c == 0 as char
                    {
                        break;
                    }
                    err_str.push(c);  
                }
                return Err(Error::MultiProcessingErr(err_str));
            }
        }
        return Err(Error::MultiProcessingErr("Unknown error occurred".to_string()))
    }
}
