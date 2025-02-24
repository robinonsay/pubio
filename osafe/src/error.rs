
use alloc::string::String;


pub trait ErrNo
{
    fn from_errno(errno: i32) -> String;
}

#[derive(Debug)]
pub enum Error
{
    IoErr(String),
    MultiProcessingErr(String),
    IpcError(String)
}

mod posix
{
use alloc::string::{String, ToString};

use core::{ffi, str::FromStr};
use crate::posix::strerror;

use super::ErrNo;
impl ErrNo for String
{
    fn from_errno(errno: i32) -> String
    {
        // thread create failed, get the status error
        let err_msg = unsafe{strerror(errno)} as *const i8;
        // Check if the error message is null
        if !err_msg.is_null()
        {
            // Get the c string
            let c_str = unsafe { ffi::CStr::from_ptr(err_msg) };
            // Convert to a &str
            let err_str = c_str.to_str()
            .map_err(|e| e.to_string());
            let err_str = match err_str
            {
                Ok(e) => e,
                Err(e) => return e,
            };
            // Convert to a String
            let err_str = String::from_str(err_str)
            .map_err(|e| e.to_string());
            let err_str = match err_str
            {
                Ok(e) => e,
                Err(e) => return e,
            };
            // Retrun the error
            return err_str;
        }
        return "Unknown Error".to_string()
    }
}
}
