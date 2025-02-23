use core::ffi;
use alloc::string::{String, ToString};

use crate::posix::printf;

use super::Printable;
use super::Error;

pub struct PosixIo;

impl Printable for PosixIo
{
    fn print(msg: &str) -> Result<usize, Error> {
        let mut msg = msg.to_string();
        msg.push(0 as char);
        let ret = unsafe {
            printf("%s\0".as_ptr() as *const ffi::c_char, msg.as_ptr())
        };
        if ret < 0
        {
            return Err(Error::IoErr("Posix: Failed to printf".to_string()));
        }
        Ok(ret as usize)
    }
    
    fn println(msg: &str) -> Result<usize, Error> {
        let mut msg = msg.to_string();
        msg.push(0 as char);
        let ret = unsafe {
            printf("%s\n\0".as_ptr() as *const ffi::c_char, msg.as_ptr())
        };
        if ret < 0
        {
            return Err(Error::IoErr("Posix: Failed to printf".to_string()));
        }
        Ok(ret as usize) 
    }
    
    fn printstr(msg: &String) -> Result<usize, Error> {
        let ret = unsafe {
            printf("%s\0".as_ptr() as *const ffi::c_char, msg.as_ptr())
        };
        if ret < 0
        {
            return Err(Error::IoErr("Posix: Failed to printf".to_string()));
        }
        Ok(ret as usize)
    }
    
    fn printstrln(msg: &String) -> Result<usize, Error> {
        let ret = unsafe {
            printf("%s\n\0".as_ptr() as *const ffi::c_char, msg.as_ptr())
        };
        if ret < 0
        {
            return Err(Error::IoErr("Posix: Failed to printf".to_string()));
        }
        Ok(ret as usize)
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;

    use super::*;

    #[test]
    fn test_print(){
        let test= "Hello World\n";
        let test_str = test.to_string();
        let result = PosixIo::printstr(&test_str);
        assert!(result.is_ok());
        let ret = result.unwrap();
        assert_eq!(ret, test_str.len());
        let result = PosixIo::printstrln(&test_str);
        assert!(result.is_ok());
        let ret = result.unwrap();
        assert_eq!(ret, test_str.len()+1);
        let result = PosixIo::print(&test);
        assert!(result.is_ok());
        let ret = result.unwrap();
        assert_eq!(ret, test_str.len());
        let result = PosixIo::println(&test);
        assert!(result.is_ok());
        let ret = result.unwrap();
        assert_eq!(ret, test_str.len()+1);
    }
}
